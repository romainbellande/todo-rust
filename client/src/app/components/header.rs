use yew::{classes, function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <header class={classes!("flex", "items-center", "bg-slate-800", "text-sky-300", "h-12")}>
      </header>
    }
}
