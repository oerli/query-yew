use yew::prelude::*;
use patternfly_yew::*;
use reqwasm::http::Request;

//TODO: change to ViewQuestionForm
use crate::answer::ViewAnswers;
use super::Question;

pub enum Msg {
    LoadQuestions(Vec<Question>),
}


#[derive(Clone, Properties, PartialEq)]
pub struct ViewQuestionsProps {
    pub session: String,
}

pub struct ViewQuestions {
    questions: Vec<Question>,
    session: String,
}

impl Component for ViewQuestions {
    type Message = Msg;
    type Properties = ViewQuestionsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let session = ctx.props().session.clone();
        ctx.link().send_future(async move {
            match Request::get(&format!("http://localhost:8787/q/{}", session)).send().await {
                Ok(r) => Msg::LoadQuestions(r.json().await.unwrap()),
                Err(_) => todo!()
            }
        });
        ViewQuestions {
            questions: vec![Question::new()],
            session: ctx.props().session.clone(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        self.questions.iter().map(|question| {
    
            html! {
                <StackItem fill=true>
                    <Card selected=true selectable=true>
                        <Text component={TextVariant::H2}>{question.question.clone()}</Text>
                        <Form>
                            <ViewAnswers answers={question.answers.clone()}/>
                        </Form>
                        <Button icon={Icon::PlusCircleIcon} label="Click" variant={Variant::Primary}/>
                    </Card>
                    <br/>
                </StackItem>
            }
        }).collect()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadQuestions(q) => {
                self.questions = q;
                true
            },
            _ => false
        }
    }
}
