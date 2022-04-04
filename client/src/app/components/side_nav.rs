use crate::router::Route;
use stylist::yew::styled_component;
use yew::{classes, html};
use yew_router::prelude::*;

#[styled_component(SideNav)]
pub fn side_nav() -> Html {
    html! {
        <nav class={classes!("min-h-screen", "w-48", "bg-slate-800", "text-sky-300", "px-2", "py-4", "space-y-4", css!("min-width: 12rem;"))}>
            <span class={classes!("px-6")}>{ "Todo App" }</span>
            <ul>
                <li>
                    <Link<Route> to={Route::Home} classes={classes!("flex", "cursor-pointer", "items-center","text-sm,", "py-4","px-6","h-12","overflow-hidden","text-ellipsis","whitespace-nowrap","hover:bg-slate-700", "transition", "duration-300", "ease-in-out", "ripple-surface-dark")}>
                     { "Home" }
                    </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}
