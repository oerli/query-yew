use serde::Deserialize;
use yew::prelude::*;
use patternfly_yew::*;
use reqwasm::http::Request;
use serde_json::json;
use gloo::storage::{LocalStorage, Storage};
use std::time::Duration;

use super::view_question_form::ViewQuestionForm;
use super::{Question, Session, QuestionOptions};
use crate::answer::Vote;

use crate::{API_URL, VOTE_KEY};

pub enum Msg {
    LoadQuestions(ViewQuestions),
    ChangeVotes(Question),
    Submit,
    ReceiveSession(Session),
    ShowToast(Toast),
}


#[derive(Properties, PartialEq)]
pub struct ViewQuestionsProps {
    pub session: String,
}

#[derive(Deserialize)]
pub struct ViewQuestions {
    questions: Vec<Question>,
    options: QuestionOptions,
    session: Session,
    #[serde(skip_deserializing)]
    vote_key: Option<Session>,
}

impl Component for ViewQuestions {
    type Message = Msg;
    type Properties = ViewQuestionsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let session = ctx.props().session.clone();

        ctx.link().send_future(async move {
            match Request::get(&format!("{}/question/{}", API_URL, session)).send().await {
                Ok(r) => Msg::LoadQuestions(r.json().await.unwrap()),
                Err(e) => {
                    Msg::ShowToast(Toast{
                        title: "Error :(".into(),
                        timeout: Some(Duration::from_secs(5)),
                        r#type: Type::Danger,
                        body: e.into(),
                        ..Default::default()
                    })
                },
            }
        });
        ViewQuestions {
            questions: vec![],
            session: Session{session: ctx.props().session.clone(), lifetime: 0},
            options: QuestionOptions { title: "Questionnaire".to_owned() },
            vote_key: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let question_list: Html = self.questions.iter().map(|question| {
            html! {
                <StackItem fill=true>
                    <Card>
                        <Form>
                            <ViewQuestionForm question={question.clone()} on_change_vote={ctx.link().callback(Msg::ChangeVotes)}/>
                        </Form>
                    </Card>
                </StackItem>
                
            }
        }).collect();

        let onclick_submit = ctx.link().callback(|_| Msg::Submit);

        html! {
            <>
                {question_list}
                <StackItem>
                    <Split gutter=true>
                        <SplitItem fill=true />
                        <SplitItem>
                            <Button icon={Icon::CheckCircle} label="Submit" variant={Variant::Primary} onclick={onclick_submit} disabled={
                                match self.vote_key {
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadQuestions(q) => {
                self.questions = q.questions;
                self.options = q.options;

                for question in &mut self.questions {
                    for answer in &mut question.answers {
                        match &answer.vote {
                            Some(_a) => (),
                            None => answer.vote = Some(Vote { vote: "false".to_owned(), answer_key: Some(answer.key.clone()), question_key: Some(question.key.clone())}),
                        }
                    }
                }
                true
            },
            Msg::ChangeVotes(question) => {
                for (index, q) in self.questions.iter().enumerate() {
                    if q.key == question.key {
                        let _ = std::mem::replace(&mut self.questions[index], question);
                        LocalStorage::set(VOTE_KEY, &self.questions).unwrap();
                        return true;
                    }
                }
                false
            },
            Msg::Submit => {
                let mut votes: Vec<Vote> = Vec::new();
                for q in &self.questions {
                    for a in &q.answers {
                        match &a.vote {
                            Some(v) => votes.push(v.clone()),
                            None => (),
                        }
                    }
                }

                let session = self.session.session.clone();
                let payload = json!(votes).to_string();
                ctx.link().send_future(async move {
                    //TODO: .json should be used, wait for reqwasm update, serde_json can be removed afterwards
                    match Request::post(&format!("{}/vote/{}", API_URL, session)).header("Content-Type", "application/json").body(payload).send().await {
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
            Msg::ReceiveSession(s) => {
                let session = self.session.session.clone();
                let key = s.session.clone();
                self.vote_key = Some(s);
                
                ctx.link().send_future(async move {
                    Msg::ShowToast(Toast{
                        title: "Vote Submitted!".into(),
                        r#type: Type::Success,
                        body: html!{
                            <>
                                <p>{"Vote Key: "}{key}</p>
                                <p>{"View Results: "}<a href={format!("/result/{}", session)}>{format!("/result/{}", session)}</a></p>
                            </>
                        },
                        ..Default::default()
                    })
                });
                true
            },
            Msg::ShowToast(t) => {
                ToastDispatcher::new().toast(t);
                false
            }
        }
    }
}
