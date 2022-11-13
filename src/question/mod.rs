use serde::{Deserialize, Serialize};

use super::header::Header;

mod view_questions;
mod create_questions;
mod create_question_form;
mod view_question_form;

pub use view_questions::ViewQuestions;
pub use create_questions::CreateQuestions;

pub use query::{Session, Question};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct QuestionOptions {
    pub title: String,
}
