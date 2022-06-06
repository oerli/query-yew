use yew::prelude::*;
use patternfly_yew::*;
use web_sys::HtmlInputElement;

use crate::answer::{Answer, CreateAnswerForm};
use super::Question;

pub enum Msg {
    AddQuestion,
    TextChanged,
    AppendAnswer(Answer),
}

#[derive(Properties, PartialEq)]
pub struct CreateQuestionFormProps {
    pub on_add_question: Callback<Question>,
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
            Msg::AddQuestion => {
                ctx.props().on_add_question.emit(std::mem::take(question));
                true
            },
            Msg::TextChanged => {
                if let Some(input) = self.my_input.cast::<HtmlInputElement>() {
                    self.question.question = input.value();
                    return true;
                }
                false
            },
            Msg::AppendAnswer(answer) => {
                self.question.answers.push(answer);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onchange = link.callback(move|_| Msg::TextChanged);
        
        let onclick_add_question = ctx.link().callback(|_| Msg::AddQuestion);

        let answer_list: Html = self.question.answers.iter().map(| answer| html! {
            <CreateAnswerForm answer={answer.clone()} on_add_answer={ctx.link().callback(Msg::AppendAnswer)}/>
        }).collect();

        html! {
            <>
                <TextInput placeholder="Add a Question" icon={TextInputIcon::Clock} ref={self.my_input.clone()} {onchange} value={self.question.question.clone()}/>
                {self.question.question.clone()}
                
                {answer_list}

                <Button icon={Icon::PlusCircleIcon} label="Add Question" variant={Variant::Primary} onclick={onclick_add_question}/>
            </>
        }
    }
}
