use yew::{classes, function_component, html};

#[function_component(Todo)]
pub fn todo() -> Html {
    html! {
      <div class={classes!("block", "p-6", "rounded-lg", "shadow-lg", "bg-slate-800", "w-48", "text-sky-300")}>
        { "Todo Card" }
      </div>
    }
}
