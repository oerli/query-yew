use yew::prelude::*;
use yew_router::prelude::*;
use patternfly_yew::*;

mod question;
mod answer;
use question::{ViewQuestions, CreateQuestions};
mod result;
use result::ViewResults;

pub const KEY: &str = "rs.query.questions";
pub const OPTIONS: &str = "rs.query.options";
pub const SESSION_KEY: &str = "rs.query.session";
pub const VOTE_KEY: &str = "rs.query.vote";

pub const API_URL: &str = "http://localhost:8787";
pub const GUI_URL: &str = "http://localhost:8080";

// TODO
// - add/remove question individual position
// - configuration options for questionnaire/questions/answers
//   - ctf style (only correct answer continue)
// - statistic options (number, count)
// - already sent questionnaire local storage (not sending same twice)
// - fix typing input to send
// - fix copy all data
// - statistic graph
// - own css/gui
// - email links (fixed links)

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    CreateQuestions,
    #[at("/result/:session")]
    ViewResults {session: String},
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
                </Stack>
            </div>
        ),
        Route::ViewResults {session} => html!(
            <div class="pf-u-p-xl">
                <Stack gutter=true>
                    <ViewResults session={session.clone()}/>
                </Stack>
            </div>
        ),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <Stack>
            <StackItem>
            <div class="pf-u-background-color-active-color-100">
            <div class="pf-u-color-light-200">
            <div class="pf-u-box-shadow-md">
            
                <Split>
                <SplitItem ></SplitItem>
                    <SplitItem fill=true>
                        <div class="pf-u-m-sm">    
                        <div class="pf-u-text-align-center">
                            <Title level={Level::H1}>{"Questionnaire"}</Title>
                        </div>
                        </div>
                    </SplitItem>
                    
                    <SplitItem>
                        <div class="pf-u-m-sm">
                        <div class="pf-u-text-align-center">
                            <a href="https://github.com/oerli/query-yew" class="pf-u-color-light-200">
                                <Title level={Level::H1}>
                                    <i class="fas fa-bug" aria-hidden="true"></i>
                                </Title>
                            </a>
                        </div>
                        </div>
                    </SplitItem>
                </Split>
            </div>
            </div>
            </div>
            </StackItem>
            <StackItem>
                <BrowserRouter>
                    <ToastViewer/>
                    <yew_router::Switch<Route> render={yew_router::Switch::render(switch)} />
                </BrowserRouter>
            </StackItem>
        </Stack>
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}