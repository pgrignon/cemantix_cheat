#[derive(Debug)]
pub struct ScoreAssociatedWords {
    pub score: f32,
    pub words: Vec<String>,
}

#[derive(Debug)]
pub struct ScoreAssociatedResponse {
    pub word: String,
    pub remaining_words: bool,
}

impl ScoreAssociatedWords {
    pub fn get_word(&mut self) -> ScoreAssociatedResponse {
        let w = self.words.pop().unwrap();
        return ScoreAssociatedResponse {
            word: w,
            remaining_words: (self.words.len() > 0),
        };
    }
}
