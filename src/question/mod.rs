use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};

use super::answer::Answer;

mod view_questions;
mod create_questions;
mod create_question_form;
mod view_question_form;

pub use view_questions::ViewQuestions;
pub use create_questions::CreateQuestions;

#[derive(Serialize, Deserialize, Default)]
pub struct Session {
    pub session: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>,
    pub key: String,
}

impl Question {
    pub fn new() -> Self {
        Question {
            question: "".to_owned(),
            answers: vec![Answer::new()],
            key: rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect::<String>(),
        }
    }
}
