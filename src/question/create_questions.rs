use serde::{Deserialize, Serialize};
use yew::prelude::*;
use patternfly_yew::*;
use reqwasm::http::Request;
use serde_json::json;
use gloo::storage::{LocalStorage, Storage};
use std::time::Duration;
use chrono::{NaiveDateTime, DateTime, Utc};

use query::Questionnaire;

use super::create_question_form::CreateQuestionForm;
use super::{Question, QuestionOptions, Session, Header};
use crate::{KEY, OPTIONS, SESSION_KEY, API_URL, GUI_URL};

pub enum Msg {
    AppendQuestion,
    ChangeQuestion(Question),
    ChangeTitle(String),
    RemoveQuestion,
    ReceiveSession(Session),
    Submit,
    ShowToast(Toast),
    ResetSession,
}

//todo repalce by quetsionnaire
#[derive(Serialize, Deserialize, Default)]
pub struct CreateQuestions {
    questions: Vec<Question>,
    options: QuestionOptions,
    session: Option<Session>,
}

impl Component for CreateQuestions {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let questions = LocalStorage::get(KEY).unwrap_or_else(|_| vec![Question::empty()]);
        let options = LocalStorage::get(OPTIONS).unwrap_or_else(|_| QuestionOptions {title: "Questionnaire".to_owned()});
        let session = LocalStorage::get(SESSION_KEY).unwrap_or_else(|_| None);

        CreateQuestions{questions, options: options, session: session}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AppendQuestion => {
                self.questions.push(Question::empty());
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
            Msg::ChangeTitle(title) => {
                self.options.title = title;
                LocalStorage::set(OPTIONS, &self.options).unwrap();
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
                let payload = json!(self).to_string();

                log::info!("Payload: {:?}", payload);

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
                    <Card >
                        <Text component={TextVariant::H2}>{format!("Question {}", index+1)}</Text>
                        <Form>
                            <CreateQuestionForm question={question.clone()} on_change_question={ctx.link().callback(Msg::ChangeQuestion)}/>
                        </Form>
                    </Card>
                </StackItem>
            }
        }).collect();

        let onclick_remove_question = ctx.link().callback(|_| Msg::RemoveQuestion);
        let onclick_add_question = ctx.link().callback(|_| Msg::AppendQuestion);
        let onclick_submit = ctx.link().callback(|_| Msg::Submit);
        let onclick_reset_session = ctx.link().callback(|_| Msg::ResetSession);
        let question_options: Html = html! {
            <>
            <StackItem>
                <Card>
                    <Text component={TextVariant::H2}>{"Options"}</Text>
                    <Split gutter=true>
                        <SplitItem>
                            <Form>
                                <TextInput placeholder="Title" onchange={ctx.link().callback(Msg::ChangeTitle)} value={self.options.title.clone()} />    
                                <Switch label="Show each Question after an other" disabled=true />
                            </Form>
                        </SplitItem>
                        <SplitItem fill=true></SplitItem>
                    </Split>  
                </Card>
  
            </StackItem>
            </>
        };

        html! {
            <>
                <Header title={self.options.title.clone()}/>
                {question_list}
                <StackItem>
                    <Split gutter=true>
                        <SplitItem fill=true />
                        <SplitItem>
                            <Button icon={Icon::MinusCircleIcon} label="Remove Question" variant={Variant::Secondary} onclick={onclick_remove_question}/>
                        </SplitItem>
                        <SplitItem>
                            <Button icon={Icon::PlusCircleIcon} label="Add Question" variant={Variant::Primary} onclick={onclick_add_question}/>
                        </SplitItem>
                        <SplitItem fill=true />
                    </Split>
                </StackItem>
                {question_options}
                <StackItem>
                <Split gutter=true>
                    <SplitItem fill=true />
                    <SplitItem>
                        {
                            match &self.session {
                                Some(s) => html!{<Button icon={Icon::MinusCircleIcon} label={format!("Reset Session {}", s.session)} variant={Variant::Secondary} onclick={onclick_reset_session}/>},
                                None => html!{},
                            }
                        }
                    </SplitItem>
                    <SplitItem>
                        <Button icon={Icon::CheckCircle} label="Submit" variant={Variant::Primary} onclick={onclick_submit} disabled={
                            match self.session {
                                Some(_) => true,
                                None => false,
                            }
                        }/>
                    </SplitItem>
                    <SplitItem fill=true />
                </Split>
                </StackItem>

            </>
        }
    }

}


