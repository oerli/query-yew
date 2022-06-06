use yew::prelude::*;
use patternfly_yew::*;
use web_sys::HtmlInputElement;

use super::Answer;

#[derive(Properties, PartialEq)]
pub struct ViewAnswersProps {
    pub answers: Vec<Answer>,
}

#[function_component(ViewAnswers)]
pub fn view_answers(ViewAnswersProps { answers }: &ViewAnswersProps) -> Html {
    answers.iter().map(|answer| html! {
        <Switch label={answer.answer.clone()}/>
    }).collect()
}