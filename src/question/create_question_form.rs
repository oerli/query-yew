use yew::prelude::*;
use patternfly_yew::*;
use web_sys::HtmlInputElement;

use crate::answer::{Answer, CreateAnswerForm};
use super::Question;

pub enum Msg {
    ChangeQuestion,
    AppendAnswer,
    ChangeAnswer(Answer),
    RemoveAnswer,
}

#[derive(Properties, PartialEq)]
pub struct CreateQuestionFormProps {
    pub on_change_question: Callback<Question>,
    pub question: Question,
}

pub struct CreateQuestionForm {
    question: Question,
    my_input: NodeRef,
}

impl Component for CreateQuestionForm {
    type Message = Msg;
    type Properties = CreateQuestionFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            question: ctx.props().question.clone(),
            my_input: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let question = &mut self.question;

        match msg {
            Msg::ChangeQuestion => {
                if let Some(input) = self.my_input.cast::<HtmlInputElement>() {
                    self.question.question = input.value();

                    ctx.props().on_change_question.emit(self.question.clone());
                }
                false
            },
            Msg::AppendAnswer => {
                self.question.answers.push(Answer::new());
                true
            },
            Msg::ChangeAnswer(answer) => {
                for (index, a) in self.question.answers.iter().enumerate() {
                    if a.key == answer.key {
                        let _ = std::mem::replace(&mut self.question.answers[index], answer);
                        ctx.props().on_change_question.emit(self.question.clone());
                        return true;
                    }
                }
                ctx.props().on_change_question.emit(self.question.clone());
                false
            },
            Msg::RemoveAnswer => {
                self.question.answers.pop();
                ctx.props().on_change_question.emit(self.question.clone());
                true 
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onchange = link.callback(move|_| Msg::ChangeQuestion);

        
        let answer_list: Html = self.question.answers.iter().map(| answer| html! {
            <CreateAnswerForm answer={answer.clone()} on_change_answer={ctx.link().callback(Msg::ChangeAnswer)}/>
        }).collect();
        
        let onclick_add_answer = ctx.link().callback(|_| Msg::AppendAnswer);
        let onclick_remove_answer = ctx.link().callback(|_| Msg::RemoveAnswer);

        let answer_options: Html = html! {
            <Split gutter=true>
                <SplitItem>
                    <Switch label="Multiple Answers possible" disabled=true/><br/>
                    <Switch label="Answers provided must be correct" disabled=true/>
                </SplitItem>
                <SplitItem fill=true>
                </SplitItem>
                <SplitItem>
                    <Button icon={Icon::MinusCircleIcon} label="Remove Answer" variant={Variant::Secondary} onclick={onclick_remove_answer}/>
                </SplitItem>
                <SplitItem>
                    <Button icon={Icon::PlusCircleIcon} label="Add Answer" variant={Variant::Primary} onclick={onclick_add_answer}/>
                </SplitItem>
            </Split>
        };

        html! {
            <>
                <TextInput placeholder="Add a Question" ref={self.my_input.clone()} {onchange} value={self.question.question.clone()} state={
                    if self.question.question.len() > 0 {
                        InputState::Default
                    } else {
                        InputState::Error
                    }
                }/>
                
                {answer_list}
                {answer_options}
            </>
        }
    }
}
