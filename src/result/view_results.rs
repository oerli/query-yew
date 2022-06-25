use std::collections::HashMap;

use yew::prelude::*;
use patternfly_yew::*;
use reqwasm::http::Request;
use serde::Deserialize;

use crate::question::{Question, Session};
use super::Score;
use crate::answer::Vote;

use crate::API_URL;

pub enum Msg {
    LoadResults(ViewResults),
}


#[derive(Clone, Properties, PartialEq)]
pub struct ViewResultsProps {
    pub session: String,
}

#[derive(Deserialize)]
pub struct ViewResults {
    questions: Vec<Question>,
    votes: Vec<Vote>,
    #[serde(skip_deserializing)]
    session: Session,
    #[serde(skip_deserializing)]
    description: HashMap<String, String>,
    #[serde(skip_deserializing)]
    count: HashMap<String, HashMap<String, HashMap<String, usize>>>,
}

impl Component for ViewResults {
    type Message = Msg;
    type Properties = ViewResultsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let session = ctx.props().session.clone();
        ctx.link().send_future(async move {
            match Request::get(&format!("{}/result/{}", API_URL, session)).send().await {
                Ok(r) => Msg::LoadResults(r.json().await.unwrap()),
                Err(_) => todo!()
            }
        });
        ViewResults {
            questions: vec![],
            session: Session{session: ctx.props().session.clone(), lifetime: 0},
            votes: vec![],
            description: HashMap::new(),
            count: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.questions.iter().map(|question| {
            let header = html_nested! {
                <TableHeader>
                    <TableColumn label={question.question.clone()}/>
                    <TableColumn label="Answer"/>
                    <TableColumn label="Count"/>
                    <TableColumn label="Total"/>
                </TableHeader>
            };
            
            let scores: Vec<Score> = question.answers.iter().filter_map(|answer| {
                match self.count.get(&question.key) {
                    Some(q) => match q.get(&answer.key) {
                        Some(a) => {
                            let mut total = 0 as usize;
                            for (_, c) in a.iter() {
                                total = total + c;
                            }
                            
                            Some(
                                a.iter().map(move |(s, c)| {
                                    Score {
                                        answer: answer.answer.clone(),
                                        vote: s.clone(),
                                        count: c.clone(),
                                        total: total.clone(),
                                    }
                                })
                            )
                        },
                        None => {
                            log::debug!("{:?}", &answer.key);
                            None
                        },
                    },
                    None => {
                        log::debug!("{:?}", &question.key);
                        None
                    },
                }
            }).flatten().collect();                

            let model: SharedTableModel<_> = scores.clone().into();
    
            html! {
                <>
                    <Table<SharedTableModel<Score>>
                        mode={TableMode::Compact}
                        header={header}
                        entries={model}
                        >
                    </Table<SharedTableModel<Score>>>
                </>
            }
        }).collect()

    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadResults(r) => {
                self.questions = r.questions;
                self.votes = r.votes;

                for question in &self.questions {
                    self.description.insert(question.key.clone(), question.question.clone());
                    for answer in &question.answers {
                        self.description.insert(answer.key.clone(), answer.answer.clone());
                    }
                }

                for vote in &self.votes {
                    match self.count.get_mut(vote.question_key.as_ref().unwrap()) {
                        Some(question) => {
                            match question.get_mut(vote.answer_key.as_ref().unwrap()) {
                                Some(v) => {
                                    match v.get_mut(&vote.vote) {
                                        Some(count) => {*count += 1;},
                                        None => {v.insert(vote.vote.clone(), 1);}
                                    }
                                },
                                None => {
                                    question.insert(vote.answer_key.as_ref().unwrap().clone(), HashMap::new());
                                    match question.get_mut(vote.answer_key.as_ref().unwrap()) {
                                        Some(v) => {
                                            v.insert(vote.vote.clone(), 1 as usize);
                                        },
                                        None => todo!()
                                    }
                                }
                            }
                        },
                        None => {
                            self.count.insert(vote.question_key.as_ref().unwrap().clone(), HashMap::new());
                            match self.count.get_mut(vote.question_key.as_ref().unwrap()) {
                                Some(question) => {
                                    match question.get_mut(vote.answer_key.as_ref().unwrap()) {
                                        Some(v) => todo!(),
                                        None => {
                                            question.insert(vote.answer_key.as_ref().unwrap().clone(), HashMap::new());
                                            match question.get_mut(vote.answer_key.as_ref().unwrap()) {
                                                Some(v) => {
                                                    v.insert(vote.vote.clone(), 1 as usize);
                                                },
                                                None => todo!()
                                            }
                                        }
                                    }
                                },
                                None => todo!()
                            }
                        }
                    }
                }
                true
            }
        }
    }
}
