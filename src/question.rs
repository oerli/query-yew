use yew::prelude::*;
use patternfly_yew::*;
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;

use super::answer::{Answer, ViewAnswers, CreateAnswers};

#[derive(Properties, PartialEq)]
pub struct QuestionList {
    session: String,
    requested: bool,
    questions: Vec<Question>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct QuestionListProperties {
    pub session: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct ViewQuestionsProps {
    pub session: String,
}

#[function_component(ViewQuestions)]
pub fn view_questions(ViewQuestionsProps { session }: &ViewQuestionsProps) -> Html {

    let questions = use_state(|| vec![]);
    {
        let questions = questions.clone();
        let session = session.clone();
        use_effect_with_deps(move |_| {
            let questions = questions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                //TODO: error handling (server not available, not found)
                let fetched_questions: Vec<Question> = Request::get(&format!("http://localhost:8787/q/{}", session)).send().await.unwrap().json().await.unwrap();
                questions.set(fetched_questions);
            });
            || ()
        }, ());
    }

    questions.iter().map(|question| html! {
        <StackItem fill=true>
            <Card selected=true selectable=true>
                <Text component={TextVariant::H2}>{question.question.clone()}</Text>
                <Form>
                    <ViewAnswers answers={question.answers.clone()}/>
                </Form>
            </Card>
            <br/>
        </StackItem>
    }).collect()
}

#[function_component(CreateQuestions)]
pub fn create_questions() -> Html {
    let questions: Vec<Question> = vec![Question{question: "Create a Question".to_owned(), answers: vec![Answer{answer:"Add an answer".to_owned(), id:"1".to_owned()}]}];

    questions.iter().enumerate().map(|(index, question)| html! {
        <StackItem fill=true>
            <Card selected=true selectable=true>
                <Text component={TextVariant::H2}>{format!("Question {}", index+1)}</Text>
                <Form>
                    <TextInput placeholder={format!("{}", question.question)}/>
                    <CreateAnswers answers={question.answers.clone()}/>
                </Form>
            </Card>
            <br/>
        </StackItem>
    }).collect()
}