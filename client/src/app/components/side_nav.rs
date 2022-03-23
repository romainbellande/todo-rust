use yew::{classes, function_component, html};
use yew_router::hooks::use_history;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(SideNav)]
pub fn side_nav() -> Html {
    // let history = use_history().unwrap();
    // let go_to_home = Callback::once(move |_| history.push(Route::Home));

    html! {
        <nav class={classes!("min-h-screen", "w-48", "bg-slate-800", "text-sky-300", "px-2", "py-4")}>
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
