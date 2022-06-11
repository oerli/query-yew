use patternfly_yew::TableRenderer;
use serde::{Deserialize, Serialize};
use yew::html;
use super::question::Question;
use super::answer::Vote;

mod view_results;
pub use view_results::ViewResults;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Score {
    pub answer: String,
    pub vote: String,
    pub count: usize,
}

impl TableRenderer for Score {
    fn render(&self, column: patternfly_yew::ColumnIndex) -> yew::Html {
        html!(
            match column.index {
                0 => self.answer.clone(),
                1 => self.vote.clone(),
                2 => self.count.to_string(),
                _ => "(unknown)".to_owned()
            }
        )
    }
}
