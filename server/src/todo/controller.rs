use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};
extern crate entity;
use entity::todo::{Entity as Todo, self};
use entity::utils::{ColumnFinder, get_expr};
use sea_orm::{entity::*, query::*, Paginator, DatabaseConnection, EntityTrait, SelectModel, FromQueryResult};
use server::{AppState, PaginatedResult, get_page_from_request, get_limit_from_request, get_search_from_request};
use super::service;
use crud::controller;

pub async fn get_paginated_result<'db, T: FromQueryResult>(
    paginator: Paginator<'db, DatabaseConnection, SelectModel<T>>,
    data: Vec<T>
) -> PaginatedResult<T> {
    let total = paginator.num_pages().await.ok().unwrap();
    let count = paginator.num_items().await.ok().unwrap();
    let page = paginator.cur_page();

    PaginatedResult {
        page,
        total,
        count,
        page_count: data.len(),
        data,
    }
}

async fn list(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;
    let page = get_page_from_request(&req);
    let limit = get_limit_from_request(&req);
    let search = get_search_from_request(&req);
    let search_items: Vec<&str> = search.split('|').collect();

    let mut finder = Todo::find();

    if search_items.len() == 3 {
        let column_name = search_items[0];
        let filter_condition = search_items[1];
        let search_value = search_items[2];

        let column = todo::Column::find_col_by_name(column_name);

        if column.is_some() {
            let expr = get_expr(column.unwrap(), filter_condition, search_value);

            if expr.is_some() {
                finder = finder.filter(expr.unwrap());
            }
        }

    }

    let paginator = finder.paginate(db, limit);

    let todos: Vec<todo::Model> = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retreive data");

    let paginated_result = get_paginated_result(paginator, todos).await;

    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(serde_json::json!(paginated_result).to_string())
}

async fn create(data: web::Data<AppState>, todo_dto: web::Json<todo::Model>) -> impl Responder {
    let response = service::create(data, todo_dto).await;
    controller::create(response)
}

pub fn controller() -> Scope {
    return web::scope("/todos")
        .route("", web::get().to(list))
        .route("", web::post().to(create));
}
