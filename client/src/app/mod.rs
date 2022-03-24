use yew::{classes, function_component, html};
use components::side_nav::SideNav;
use components::header::Header;
use yew_router::prelude::*;
use crate::router::{Route, switch};

pub mod home;
mod components;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class={classes!("bg-slate-700", "flex")}>
                <SideNav />
                <div class={classes!("min-h-screen", "flex", "flex-col", "flex-auto")}>
                    <Header />
                    <div class={classes!("flex-auto")}>
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </div>
            </div>
        </BrowserRouter>
    }
}
