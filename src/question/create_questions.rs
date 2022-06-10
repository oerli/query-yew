use yew::prelude::*;
use patternfly_yew::*;
use reqwasm::http::Request;
use serde_json::json;
use gloo::storage::{LocalStorage, Storage};

use super::create_question_form::CreateQuestionForm;
use super::{Question, Session};
use crate::{KEY, SESSION_KEY, API_URL, GUI_URL};

pub enum Msg {
    AppendQuestion,
    ChangeQuestion(Question),
    RemoveQuestion,
    ReceiveSession(Session),
    Submit
}

pub struct CreateQuestions {
    questions: Vec<Question>,
    session: Option<Session>,
}

impl Component for CreateQuestions {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        // CreateQuestions{questions: vec![Question{question: "Create a Question".to_owned(), answers: vec![Answer{answer:"Add an answer".to_owned(), id:"1".to_owned()}]}]}
        let questions = LocalStorage::get(KEY).unwrap_or_else(|_| vec![Question::new()]);
        let session = LocalStorage::get(SESSION_KEY).unwrap_or_else(|_| None);

        CreateQuestions{questions, session: session}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AppendQuestion => {
                self.questions.push(Question::new());
                LocalStorage::set(KEY, &self.questions).unwrap();
                true
            },
            Msg::ChangeQuestion(question) => {
                for (index, q) in self.questions.iter().enumerate() {
                    if q.key == question.key {
                        let _ = std::mem::replace(&mut self.questions[index], question);
                        LocalStorage::set(KEY, &self.questions).unwrap();
                        return true;
                    }
                }
                false
            },
            Msg::RemoveQuestion => {
                self.questions.pop();
                LocalStorage::set(KEY, &self.questions).unwrap();
                true
            },
            Msg::ReceiveSession(s) => {
                self.session = Some(s);
                LocalStorage::set(SESSION_KEY, &self.session).unwrap();
                true
            },
            Msg::Submit => {
                let payload = json!(self.questions).to_string();
                ctx.link().send_future(async {

                    //TODO: .json should be used, wait for reqwasm update, serde_json can be removed afterwards
                    match Request::post(&format!("{}/question", API_URL)).header("Content-Type", "application/json").body(payload).send().await {
                        Ok(r) => {
                            Msg::ReceiveSession(r.json().await.unwrap())
                        },
                        Err(e) => {
                            log::debug!("{:?}", e);
                            todo!()
                        }
                    }
                });
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
                            <CreateQuestionForm question={question.clone()} on_change_question={ctx.link().callback(Msg::ChangeQuestion)}/>
                        </Form>
                    </Card>
                    <br/>
                </StackItem>
            }
        }).collect();
        
        let onclick_remove_question = ctx.link().callback(|_| Msg::RemoveQuestion);
        
        let onclick_submit = ctx.link().callback(|_| Msg::Submit);

        let session = match &self.session {
            Some(s) => {html!(
                
                <PopoverPopup orientation={Orientation::Bottom} header={html!(<Title level={Level::H2}>{"Session"}</Title>)}>
                    <a href={format!("{}/{}", GUI_URL, s.session)}>{format!("{}/{}", GUI_URL, s.session)}</a>
                </PopoverPopup>
            )},
            None => html!()
        };
        
        let onclick_add_question = ctx.link().callback(|_| Msg::AppendQuestion);

        html! {
            <>
                {question_list}
                <StackItem>
                    <Button icon={Icon::PlusCircleIcon} label="Add Question" variant={Variant::Primary} onclick={onclick_add_question}/>
                    <Button icon={Icon::MinusCircleIcon} label="Remove Question" variant={Variant::Secondary} onclick={onclick_remove_question}/>
                </StackItem>
                <StackItem>
                    <Button icon={Icon::CheckCircle} label="Submit" variant={Variant::Primary} onclick={onclick_submit}/>
                </StackItem>
                {session}
            </>
        }
    }

}


