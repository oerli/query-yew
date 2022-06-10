use gloo::console::{debug, externs::debug};
use yew::prelude::*;
use patternfly_yew::*;
use web_sys::HtmlInputElement;

use super::{Answer, Vote};

pub enum Msg {
    ChangeVote,
}

#[derive(Properties, PartialEq)]
pub struct ViewAnswerFormProps {
    pub on_change_vote: Callback<Answer>,
    pub answer: Answer,
}

pub struct ViewAnswerForm {
    answer: Answer,
    my_input: NodeRef,
}

impl Component for ViewAnswerForm {
    type Message = Msg;
    type Properties = ViewAnswerFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            answer: ctx.props().answer.clone(),
            my_input: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let answer = &mut self.answer;       

        match msg {
            Msg::ChangeVote => {
                if let Some(input) = self.my_input.cast::<HtmlInputElement>() {
                    match &mut self.answer.vote {
                        &mut Some(ref mut vote) => {
                            match input.checked() {
                                true => {
                                    vote.vote = "false".to_owned();
                                    input.set_checked(false);
                                },
                                false => {
                                    vote.vote = "true".to_owned();
                                    input.set_checked(true);
                                }
                            }

                        },
                        &mut None => {
                            match input.checked() {
                                true => self.answer.vote = {
                                    input.set_checked(false);
                                    Some(Vote {vote: "false".to_owned(), answer_key: Some(self.answer.key.clone()), question_key: None})
                                },
                                false => {
                                    input.set_checked(true);
                                    self.answer.vote = Some(Vote {vote: "true".to_owned(), answer_key: Some(self.answer.key.clone()), question_key: None})
                                },
                            }
                        }
                    } 
                    
                    ctx.props().on_change_vote.emit(self.answer.clone());
                }
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(|_| Msg::ChangeVote);
        let checked = match &self.answer.vote {
            Some(vote) => {
                match vote.vote.as_str() {
                    "true" => true,
                    _ => false,
                }
            },
            _ => false,
        };

        html! {
            <>
                <Switch ref={self.my_input.clone()} {on_change} label={self.answer.answer.clone()} {checked}/>
            </>
        }
    }
}
