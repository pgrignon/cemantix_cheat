use rand::rngs::mock::StepRng;
use rand::Rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

use crate::api::SimilarWords;

use self::score_associated_words::ScoreAssociatedWords;

mod score_associated_words;
mod utils;

#[derive(Debug)]
pub struct History {
    pub found: bool,
    pub words_submitted: Vec<String>,
    pub words_pool: Vec<String>,
    pub cool_score: f32,
    pub hot_score: f32,
    pub cool_buffer: Vec<ScoreAssociatedWords>,
    pub hot_buffer: Vec<ScoreAssociatedWords>,
}

impl Default for History {
    fn default() -> Self {
        return History {
            found: false,
            words_submitted: Vec::new(),
            words_pool: Vec::new(),
            cool_score: -100.0,
            hot_score: -100.0,
            cool_buffer: Vec::new(),
            hot_buffer: Vec::new(),
        };
    }
}

impl History {
    pub fn build_words_pool(&mut self, filepath: String, randomize: bool) {
        if let Ok(lines) = utils::read_lines(filepath) {
            for line in lines {
                if let Ok(word) = line {
                    self.words_pool.push(word);
                }
            }
        }
        if randomize {
            let mut rng = rand::thread_rng();
            let mut rng = StepRng::new(rng.gen::<u64>(), 13);
            let mut irs = Irs::default();
            irs.shuffle(&mut self.words_pool, &mut rng)
                .expect("Error while randomizing list order");
        }
    }

    pub fn add_word_to_history(&mut self, w: String) {
        self.words_submitted.push(w);
    }

    pub fn check_if_word_submitted(&self, word: &String) -> bool {
        if self.words_submitted.contains(&word) {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_hot(&self) -> bool {
        if self.hot_buffer.len() > 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_cool(&self) -> bool {
        if self.cool_buffer.len() > 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_hottest_word(&mut self) -> String {
        let mut index: usize = usize::default();
        let mut score = -100.0;
        for (i, scw) in self.hot_buffer.iter().enumerate() {
            if scw.score > score {
                score = scw.score.clone();
                index = i.clone();
            }
        }
        let r = self.hot_buffer[index].get_word();
        if !r.remaining_words {
            self.hot_buffer.remove(index);
        }
        return r.word;
    }

    pub fn get_coolest_word(&mut self) -> String {
        let mut index: usize = usize::default();
        let mut score = -100.0;
        for (i, scw) in self.cool_buffer.iter().enumerate() {
            if scw.score > score {
                score = scw.score.clone();
                index = i.clone();
            }
        }
        let r = self.cool_buffer[index].get_word();
        if !r.remaining_words {
            self.cool_buffer.remove(index);
        }
        return r.word;
    }

    pub fn get_classic_word(&mut self) -> String {
        return self.words_pool.pop().unwrap();
    }

    pub fn add_hot_words_to_buffer(&mut self, score: f32, s_words: SimilarWords) {
        let score_associated_words = ScoreAssociatedWords {
            score,
            words: s_words.similar_words,
        };
        self.hot_buffer.push(score_associated_words);
    }

    pub fn add_cool_words_to_buffer(&mut self, score: f32, s_words: SimilarWords) {
        let score_associated_words = ScoreAssociatedWords {
            score,
            words: s_words.similar_words,
        };
        self.cool_buffer.push(score_associated_words);
    }
}
