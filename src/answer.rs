use yew::prelude::*;
use patternfly_yew::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Answer {
    pub answer: String,
    pub id: String,
}

#[derive(Properties, PartialEq)]
pub struct AnswersProps {
    pub answers: Vec<Answer>,
}

#[function_component(ViewAnswers)]
pub fn view_answers(AnswersProps { answers }: &AnswersProps) -> Html {
    answers.iter().map(|answer| html! {
        <Switch label={answer.answer.clone()}/>
    }).collect()
}

#[function_component(CreateAnswers)]
pub fn create_answers(AnswersProps { answers }: &AnswersProps) -> Html {
    answers.iter().enumerate().map(|(index, answer)| html! {
        <FormGroup label={format!("Answer {}", index+1)}>
            <TextInput placeholder={format!("{}", answer.answer)} icon={TextInputIcon::Search}/>
        </FormGroup>
    }).collect()
}