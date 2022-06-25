use yew::prelude::*;
use patternfly_yew::*;
use reqwasm::http::Request;
use serde_json::json;
use gloo::storage::{LocalStorage, Storage};
use std::time::Duration;
use chrono::{NaiveDateTime, DateTime, Utc};

use super::create_question_form::CreateQuestionForm;
use super::{Question, Session};
use crate::{KEY, SESSION_KEY, API_URL, GUI_URL};

pub enum Msg {
    AppendQuestion,
    ChangeQuestion(Question),
    RemoveQuestion,
    ReceiveSession(Session),
    Submit,
    ShowToast(Toast),
    ResetSession,
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
                let session = s.session.clone();
                let lifetime = s.lifetime.clone();
                self.session = Some(s);
                LocalStorage::set(SESSION_KEY, &self.session).unwrap();

                ctx.link().send_future(async move {
                    Msg::ShowToast(Toast{
                        title: "Questions Submitted!".into(),
                        r#type: Type::Success,
                        body: html!{
                            <>
                                <p>{"Session Key: "}{session.clone()}</p>
                                <p>{"Valid until: "}{DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(lifetime as i64, 0), Utc).format("%-d %b %Y %H:%M UTC")}</p>
                                <p>{"View Questions: "}<a href={format!("{}/{}", GUI_URL, session)}>{format!("{}/{}", GUI_URL, session)}</a></p>
                            </>
                        },
                        ..Default::default()
                    })
                });

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
                            Msg::ShowToast(Toast{
                                title: "Error :(".into(),
                                timeout: Some(Duration::from_secs(15)),
                                r#type: Type::Danger,
                                body: e.into(),
                                ..Default::default()
                            })
                        }
                    }
                });
                true
            },
            Msg::ShowToast(t) => {
                ToastDispatcher::new().toast(t);
                false
            },
            Msg::ResetSession => {
                self.session = None;
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
        
        let onclick_add_question = ctx.link().callback(|_| Msg::AppendQuestion);

        let onclick_reset_session = ctx.link().callback(|_| Msg::ResetSession);

        html! {
            <>
                {question_list}
                <StackItem>
                    <Split gutter=true>
                        <SplitItem><Button icon={Icon::PlusCircleIcon} label="Add Question" variant={Variant::Primary} onclick={onclick_add_question}/></SplitItem>
                        <SplitItem><Button icon={Icon::MinusCircleIcon} label="Remove Question" variant={Variant::Secondary} onclick={onclick_remove_question}/></SplitItem>
                    </Split>
                </StackItem>
                <StackItem>
                    <Split gutter=true>
                        <SplitItem><Button icon={Icon::CheckCircle} label="Submit" variant={Variant::Primary} onclick={onclick_submit} disabled={
                                match self.session {
                                    Some(_) => true,
                                    None => false,
                                }
                            }/>
                        </SplitItem>
                        <SplitItem>
                            {
                                match &self.session {
                                    Some(s) => html!{<Button icon={Icon::MinusCircleIcon} label={format!("Reset Session {}", s.session)} variant={Variant::Primary} onclick={onclick_reset_session}/>},
                                    None => html!{},
                                }
                            }
                        </SplitItem>
                    </Split>    
                </StackItem>
            </>
        }
    }

}


