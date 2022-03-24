mod components;

use yew::{
    Html,
    classes,
    function_component,
    html,
    use_state, use_effect_with_deps
};
use components::todo::Todo;
use crate::entities::todo::Todo as TodoEntity;
use crate::entities::PaginatedResult;
use reqwasm::http::{Request};
use wasm_bindgen_futures::spawn_local;

#[function_component(Home)]
pub fn home() -> Html {
    let todos= use_state(|| PaginatedResult::<TodoEntity> {
        count: 0,
        data: Vec::<TodoEntity>::new(),
        page: 0,
        page_count: 0,
        total: 0,
    });

    {
        let todos = todos.clone();

        use_effect_with_deps(move |_| {

            spawn_local(async move {
                let fetched_todos: PaginatedResult<TodoEntity> = Request::get("https://api.todo.localhost/api/v1/todos")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                log::info!("{:#?}", fetched_todos);
                todos.set(fetched_todos);
            });
            || ()
        }, ());
    }

    let todos_list = todos.data.iter().map(|_| html!{
        <div class={classes!("m-4")}>
            <Todo />
        </div>
    }).collect::<Html>();

    html! {
        <div class={classes!("p-4", "flex", "flex-wrap")}>
            {
                todos_list
            }
        </div>
    }
}
