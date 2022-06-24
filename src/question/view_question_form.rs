use yew::prelude::*;
use patternfly_yew::*;

use crate::answer::{Answer, ViewAnswerForm};
use super::Question;

pub enum Msg {
    ChangeVote(Answer),
}

#[derive(Properties, PartialEq)]
pub struct ViewQuestionFormProps {
    pub on_change_vote: Callback<Question>,
    pub question: Question,
}

pub struct ViewQuestionForm {
    question: Question,
}

impl Component for ViewQuestionForm {
    type Message = Msg;
    type Properties = ViewQuestionFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            question: ctx.props().question.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let question = &mut self.question;

        match msg {
            Msg::ChangeVote(answer) => {
                for (index, a) in self.question.answers.iter().enumerate() {
                    if a.key == answer.key {
                        let _ = std::mem::replace(&mut self.question.answers[index], answer);
                        ctx.props().on_change_vote.emit(self.question.clone());
                        return true;
                    }
                }
                ctx.props().on_change_vote.emit(self.question.clone());
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let answer_list: Html = self.question.answers.iter().map(| answer| html! {
            <ViewAnswerForm answer={answer.clone()} on_change_vote={ctx.link().callback(Msg::ChangeVote)}/>
        }).collect();
        
        html! {
            <>
                <Text component={TextVariant::H2}>{self.question.question.clone()}</Text>
                {answer_list}
            </>
        }
    }
}
