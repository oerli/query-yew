use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};

mod create_answer_form;
mod view_answer_form;

pub use create_answer_form::CreateAnswerForm;
pub use view_answer_form::ViewAnswerForm;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Answer {
    pub answer: String,
    pub key: String,
    #[serde(skip_serializing)]
    pub vote: Option<Vote>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Vote {
    pub vote: String,
    pub answer_key: Option<String>,
    pub question_key: Option<String>,
}

impl Answer {
    pub fn new() -> Self {
        Self {
            answer: "".to_owned(),
            key: rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect::<String>(),
            vote: None,
        }
    }
}
