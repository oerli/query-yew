use yew::prelude::*;
use yew_router::prelude::*;
use patternfly_yew::*;

mod question;
mod answer;
use question::{ViewQuestions, CreateQuestions};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    CreateQuestions,
    #[at("/:session")]
    ViewQuestions {session: String},
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::CreateQuestions => html!(
            <div class="pf-u-p-xl">
                <Stack gutter=true>
                    <CreateQuestions/>
                </Stack>
            </div>),
        Route::ViewQuestions {session} => html!(
            <div class="pf-u-p-xl">
                <Stack gutter=true>
                    <ViewQuestions session={session.clone()}/>
                    // <Questionnaire session={session.clone()}/>
                </Stack>
            </div>
        ),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <yew_router::Switch<Route> render={yew_router::Switch::render(switch)} />
        </BrowserRouter>
    }
}

pub fn main() {
    yew::start_app::<App>();
}