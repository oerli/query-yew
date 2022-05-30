use yew::prelude::*;
use patternfly_yew::*;
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;

use super::answer::{Answer, ViewAnswers, CreateAnswers};

#[derive(Clone, Properties, PartialEq)]
pub struct ListQuestionsProps {
    pub questions: Vec<Question>,
    on_click: Callback<Question>
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct ViewQuestionsProps {
    pub session: String,
}

#[function_component(ViewQuestions)]
pub fn view_questions(ViewQuestionsProps { session }: &ViewQuestionsProps) -> Html {

    let questions = use_state(|| vec![]);
    {
        let questions = questions.clone();
        let session = session.clone();
        use_effect_with_deps(move |_| {
            let questions = questions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                //TODO: error handling (server not available, not found)
                let fetched_questions: Vec<Question> = Request::get(&format!("http://localhost:8787/q/{}", session)).send().await.unwrap().json().await.unwrap();
                questions.set(fetched_questions);
            });
            || ()
        }, ());
    }

    let selected_question = use_state(|| None);

    let on_question_select = {
        let selected_question = selected_question.clone();
        Callback::from(move |question: Question| {
            selected_question.set(Some(question))
        })
    };

    let details = selected_question.as_ref().map(|question| html! {
        <h1>{question.question.clone()}</h1>
    });

    html! {
        <>
            <ListQuestions questions={(*questions).clone()} on_click={on_question_select.clone()}/>
            { for details}
            <StackItem>
                <Button icon={Icon::PlusCircleIcon} label="Submit" variant={Variant::Primary}/>
            </StackItem>
        </>
    }
}

#[function_component(ListQuestions)]
pub fn list_questions(ListQuestionsProps { questions, on_click }: &ListQuestionsProps) -> Html {
    let on_click = on_click.clone();
    questions.iter().map(|question| {
        let on_question_select = {
            let on_click = on_click.clone();
            let question = question.clone();
            Callback::from(move |_| {
                on_click.emit(question.clone())
            })
        };

        html! {
            <StackItem fill=true>
                <Card selected=true selectable=true>
                    <Text component={TextVariant::H2}>{question.question.clone()}</Text>
                    <Form>
                        <ViewAnswers answers={question.answers.clone()}/>
                    </Form>
                    <Button icon={Icon::PlusCircleIcon} label="Click" variant={Variant::Primary} onclick={on_question_select}/>
                </Card>
                <br/>
            </StackItem>
        }
    }).collect()
}

#[function_component(CreateQuestions)]
pub fn create_questions() -> Html {
    let questions: Vec<Question> = vec![Question{question: "Create a Question".to_owned(), answers: vec![Answer{answer:"Add an answer".to_owned(), id:"1".to_owned()}]}];

    let selected_question = use_state(|| None);

    let on_click = {
        let selected_question = selected_question.clone();
        Callback::from(move |question: Question| {
            selected_question.set(Some(question))
        })
    };

    

    questions.iter().enumerate().map(|(index, question)| {
        let details = selected_question.as_ref().map(|question| html! {
            <h1>{question.question.clone()}</h1>
        });

        let on_question_select = {
            let on_click = on_click.clone();
            let mut question = question.clone();
            question.answers.push(Answer { answer: "".to_owned(), id: "".to_owned() });
            Callback::from(move |_| {
                on_click.emit(question.clone())
            })
        };

        html! {
            <StackItem fill=true>
                <Card selected=true selectable=true>
                    <Text component={TextVariant::H2}>{format!("Question {}", index+1)}</Text>
                    <Form>
                        <TextInput placeholder={format!("{}", question.question)}/>
                        <CreateAnswers answers={question.answers.clone()}/>
                        { for details }
                        <Button icon={Icon::PlusCircleIcon} label="Click" variant={Variant::Primary} onclick={on_question_select}/>
                    </Form>
                </Card>
                <br/>
            </StackItem>
        }
    }).collect()
}