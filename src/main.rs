use api::get_similar_words;
use config::build_config;
use std::{thread, time};
use thirtyfour::prelude::*;

mod history;

mod api;
mod config;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Build config
    let config = build_config();
    let mut h = history::History::default();
    h.build_words_pool(config.words_path, config.randomize_starting_list);

    // Create driver
    let mut caps = DesiredCapabilities::chrome();
    let opt = caps.add_chrome_arg("kiosk");
    match opt {
        Ok(_v) => println!("Kiosk mode enabled"),
        Err(_e) => println!("Kiosk mode not enabled"),
    };
    let driver = WebDriver::new(&config.chromedriver_path, caps).await?;

    // Navigate to Cemantix and init used elements
    driver.goto(&config.url).await?;
    let welcome_div = driver.find(By::Id(&config.welcome_div_id)).await?;
    welcome_div.click().await?;
    let elem_form = driver.find(By::Id(&config.form_div_id)).await?;
    let elem_text = elem_form.find(By::Id(&config.form_input_id)).await?;
    let elem_button = elem_form.find(By::Id(&config.form_button_id)).await?;
    let elem_board_cool = driver.find(By::XPath(&config.cool_score_xpath)).await?;
    let elem_board_hot = driver.find(By::XPath(&config.hot_score_xpath)).await?;

    h.cool_score = elem_board_cool.text().await?.parse::<f32>().unwrap();
    h.hot_score = elem_board_hot.text().await?.parse::<f32>().unwrap();

    while !h.found {
        // Choose word to be submitted depending on buffer statuses
        let word;
        if h.is_hot() {
            word = h.get_hottest_word();
        } else {
            if h.is_cool() {
                word = h.get_coolest_word();
            } else {
                word = h.get_classic_word();
            }
        }

        // Test if word has already been submitted or not
        if !h.check_if_word_submitted(&word) {
            // Submit word
            elem_text.send_keys(&word).await?;
            elem_button.wait_until().enabled().await?;
            elem_button.click().await?;

            // Wait
            elem_button.wait_until().enabled().await?;

            // Check score
            let elem_score_text = elem_form
                .query(By::XPath(&config.form_score_guess_xpath))
                .without_text("")
                .first()
                .await?;
            let score = unwrap_score(elem_score_text.text().await);
            if score == 100.0 {
                h.found = true;
            } else {
                h.add_word_to_history(word.clone());
                if score > h.cool_score {
                    let endpoint = format!("{}{}", config.api_similar_endpoint, word);
                    let s_words = get_similar_words(endpoint).await.unwrap();
                    if score > h.hot_score {
                        h.add_hot_words_to_buffer(score, s_words)
                    } else {
                        h.add_cool_words_to_buffer(score, s_words)
                    }
                }
            }
        }
    }

    // Wait
    let ten_secs = time::Duration::from_millis(10000);
    thread::sleep(ten_secs);

    // Close the browser.
    driver.quit().await?;

    Ok(())
}

fn unwrap_score(score: Result<String, WebDriverError>) -> f32 {
    match score {
        Ok(v) => {
            return v.parse::<f32>().unwrap();
        }
        Err(_e) => {
            return -100.0;
        }
    }
}
