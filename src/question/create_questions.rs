use yew::prelude::*;
use patternfly_yew::*;
use serde::{Deserialize};
use reqwasm::http::Request;
use serde_json::json;
use gloo::storage::{LocalStorage, Storage};

use super::create_question_form::CreateQuestionForm;
use super::Question;

pub enum Msg {
    AppendQuestion(Question),
    RemoveQuestion,
    ReceiveSession(Session)
}

const KEY: &str = "dev.pages.questionnaire.questions";
const API_URL: &str = "http://localhost:8787";
const GUI_URL: &str = "http://localhost:8080";

pub struct CreateQuestions {
    questions: Vec<Question>,
    session: Option<Session>,
}

#[derive(Deserialize)]
pub struct Session {
    session: String,
}

impl Component for CreateQuestions {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        // CreateQuestions{questions: vec![Question{question: "Create a Question".to_owned(), answers: vec![Answer{answer:"Add an answer".to_owned(), id:"1".to_owned()}]}]}
        let questions = LocalStorage::get(KEY).unwrap_or_else(|_| vec![Question::new()]);

        CreateQuestions{questions, session: None}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AppendQuestion(question) => {
                // self.questions.push(Question{question: "Create a Question".to_owned(), answers: vec![Answer{answer:"Add an answer".to_owned(), id:"1".to_owned()}]});
                for (index, q) in self.questions.iter().enumerate() {
                    if q.key == question.key {
                        let _ = std::mem::replace(&mut self.questions[index], question);
                        LocalStorage::set(KEY, &self.questions).unwrap();
                        return true;
                    }
                }
                
                self.questions.push(question);
                LocalStorage::set(KEY, &self.questions).unwrap();
                true
            },
            Msg::RemoveQuestion => {
                self.questions.pop();
                if self.questions.len() == 0 {
                    self.questions.push(Question::new())
                }
                LocalStorage::set(KEY, &self.questions).unwrap();
                true
            },
            Msg::ReceiveSession(s) => {
                self.session = Some(s);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let question_list: Html = self.questions.iter().enumerate().map(|(index, question)| {

            html! {
                <StackItem fill=true>
                    <Card selected=true selectable=true>
                        <Text component={TextVariant::H2}>{format!("Question {}", index+1)}</Text>
                        <Form>
                            <CreateQuestionForm question={question.clone()} on_add_question={ctx.link().callback(Msg::AppendQuestion)}/>
                        </Form>
                    </Card>
                    <br/>
                </StackItem>
            }
        }).collect();
        
        let onclick_remove_question = ctx.link().callback(|_| Msg::RemoveQuestion);
        let payload = self.questions.clone();
        let onclick_submit = ctx.link().callback_future( move |_| {
                let payload = json!(payload).to_string();

                async move {
                    //TODO: .json should be used, wait for reqwasm update, serde_json can be removed afterwards
                    match Request::post(&format!("{}/q", API_URL)).header("Content-Type", "application/json").body(payload).send().await {
                        Ok(r) => Msg::ReceiveSession(r.json().await.unwrap()),
                        Err(_) => todo!()
                    }
                }
            }
        );

        let session = match &self.session {
            Some(s) => {html!(
                
                <PopoverPopup orientation={Orientation::Bottom} header={html!(<Title level={Level::H2}>{"Session"}</Title>)}>
                    {format!("{}/{}", GUI_URL, s.session)}
                </PopoverPopup>
            )},
            None => html!()
        };

        html! {
            <>
                {question_list}
                <StackItem>
                    <Button icon={Icon::MinusCircleIcon} label="Remove Question" variant={Variant::Secondary} onclick={onclick_remove_question}/>
                    <Button icon={Icon::PlusCircleIcon} label="Submit" variant={Variant::Primary} onclick={onclick_submit}/>
                </StackItem>
                {session}
            </>
        }
    }

}


