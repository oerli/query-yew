use yew::prelude::*;
use yew_router::prelude::*;
use patternfly_yew::*;

mod question;
mod answer;
use question::{ViewQuestions, CreateQuestions};

pub const KEY: &str = "dev.pages.questionnaire.questions";
pub const SESSION_KEY: &str = "dev.pages.questionnaire.session";

pub const API_URL: &str = "http://127.0.0.1:8787";
pub const GUI_URL: &str = "http://127.0.0.1:8080";

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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}