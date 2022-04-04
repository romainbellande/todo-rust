use crate::router::{switch, Route};
use components::header::Header;
use components::side_nav::SideNav;
use yew::{classes, function_component, html};
use yew_router::prelude::*;

mod components;
pub mod home;

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
