use yew::prelude::*;
use patternfly_yew::*;

pub enum Msg {
    ChangeTitle(String),
}

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: String,
}

pub struct Header {
    title: String,
}

impl Component for Header {
    type Message = Msg;
    type Properties = HeaderProps;

    fn create(ctx: &Context<Self>) -> Self {
        Header {
            title: ctx.props().title.clone()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeTitle(title) => {
                self.title = title.clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <StackItem>
                <div class="pf-u-background-color-active-color-100">
                <div class="pf-u-color-light-200">
                <div class="pf-u-box-shadow-md">
                
                    <Split>
                    <SplitItem ></SplitItem>
                        <SplitItem fill=true>
                            <div class="pf-u-m-sm">    
                            <div class="pf-u-text-align-center">
                                <Title level={Level::H1}>{self.title.clone()}</Title>
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
            </>
        }
    }
}
