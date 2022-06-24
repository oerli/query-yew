use patternfly_yew::TableRenderer;
use serde::{Deserialize, Serialize};
use yew::html;
use patternfly_yew::*;

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
        
        match column.index {
            0 => html!{{self.answer.clone()}},
            1 => {
                match self.vote.as_str() {
                    "true" => html!{<Label color={Color::Green} icon={Icon::CheckCircle} label={"true"}/>},
                    "false" => html!{<Label color={Color::Red} icon={Icon::MinusCircleIcon} label={"false"}/>},
                    _ => html!{{self.vote.clone()}},
                }
            },
            2 => html!{{self.count.to_string()}},
            _ => html!{{"(unknown)"}}
        }
        
    }
}
