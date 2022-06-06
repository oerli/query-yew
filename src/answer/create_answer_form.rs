use yew::prelude::*;
use patternfly_yew::*;
use web_sys::HtmlInputElement;

use super::Answer;

pub enum Msg {
    AddAnswer,
    TextChanged,
}

#[derive(Properties, PartialEq)]
pub struct CreateAnswerFormProps {
    pub on_add_answer: Callback<Answer>,
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
            Msg::AddAnswer => {
                ctx.props().on_add_answer.emit(std::mem::take(answer));
                true
            },
            Msg::TextChanged => {
                if let Some(input) = self.my_input.cast::<HtmlInputElement>() {
                    self.answer.answer = input.value();
                    return true;
                }
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onchange = link.callback(move|_| Msg::TextChanged);

        let onclick_add_answer = ctx.link().callback(|_| Msg::AddAnswer);

        html! {
            <>
                <TextInput placeholder="Add an Answer" icon={TextInputIcon::Clock} ref={self.my_input.clone()} {onchange} value={self.answer.answer.clone()}/>
                {self.answer.answer.clone()}
                
                <Button icon={Icon::PlusCircleIcon} label="Add Answer" variant={Variant::Primary} onclick={onclick_add_answer}/>
            </>
        }
    }
}
