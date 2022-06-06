use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};

mod create_answer_form;
mod view_answers;

pub use create_answer_form::CreateAnswerForm;
pub use view_answers::ViewAnswers;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Answer {
    pub answer: String,
    pub key: String,
}

impl Answer {
    pub fn new() -> Self {
        Self {
            answer: "".to_owned(),
            key: rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect::<String>(),
        }
    }
}
