use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{AppRoute};

pub struct Navbar { 
    link: ComponentLink<Self>,
    is_active: bool,
}
pub enum Msg { 
    Toggle
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { 
            link: _link,
            is_active: false,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg { 
            Msg::Toggle => { 
                self.is_active = !self.is_active;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <header class="navbar">
                <ul>
                    <li>
                        <RouterAnchor<AppRoute> route=AppRoute::Home><a>{"Home"}</a></RouterAnchor<AppRoute>>
                    </li>
                    <li>
                        <RouterAnchor<AppRoute> route=AppRoute::About><a>{"Blog"}</a></RouterAnchor<AppRoute>>
                    </li>
                </ul>
            </header>
            </>
        }
    }
}

