use yew::prelude::*;
use patternfly_yew::*;
use web_sys::HtmlInputElement;

use super::Answer;

pub enum Msg {
    ChangeAnswer,
}

#[derive(Properties, PartialEq)]
pub struct CreateAnswerFormProps {
    pub on_change_answer: Callback<Answer>,
    pub answer: Answer,
}

pub struct CreateAnswerForm {
    answer: Answer,
    my_input: NodeRef,
}

impl Component for CreateAnswerForm {
    type Message = Msg;
    type Properties = CreateAnswerFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            answer: ctx.props().answer.clone(),
            my_input: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let answer = &mut self.answer;       

        match msg {
            Msg::ChangeAnswer => {
                if let Some(input) = self.my_input.cast::<HtmlInputElement>() {
                    self.answer.answer = input.value();
                    
                    ctx.props().on_change_answer.emit(self.answer.clone());
                }
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(|_| Msg::ChangeAnswer);

        html! {
            <>
                <TextInput placeholder="Add an Answer" icon={TextInputIcon::Clock} ref={self.my_input.clone()} {onchange} value={self.answer.answer.clone()}/>
            </>
        }
    }
}
