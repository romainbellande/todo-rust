#![allow(unused_must_use)]

use serde::{Serialize, Deserialize};
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use migration::{Migrator, MigratorTrait};
use std::env;
use actix_web::{web, HttpRequest};

const DEFAULT_PAGE: u8 = 1;
const DEFAULT_LIMIT: u8 = 20;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub fn setup_logs() {
    std::env::set_var("RUST_LOG", "actix_web=warn");
    fast_log::init(fast_log::config::Config::new().console());
}

pub async fn setup_database() -> DatabaseConnection {
    let database_url= env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    log::info!("linking database...");

    let mut conn_options = ConnectOptions::new(database_url.to_owned());

    conn_options.sqlx_logging(false);

    let db: DatabaseConnection = Database::connect(conn_options).await.expect("can't connect to database");

    Migrator::up(&db, None).await.unwrap();

    log::info!("linking database successful!");

    db
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub s: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedResult<T> {
    pub data: T,
    pub count: usize,
    pub total: usize,
    pub page: usize,
    pub page_count: usize,
}

pub fn get_params_from_request(req: &HttpRequest) -> web::Query<Params> {
    return web::Query::<Params>::from_query(req.query_string()).unwrap();
}

pub fn get_page_from_request(req: &HttpRequest) -> usize {
    let params = get_params_from_request(req);

    return if params.page.is_some() {
        params.page.unwrap()
    } else {
        DEFAULT_PAGE.into()
    };
}

pub fn get_limit_from_request(req: &HttpRequest) -> usize {
    let params = get_params_from_request(req);

    return if params.limit.is_some() {
        params.limit.unwrap()
    } else {
        DEFAULT_LIMIT.into()
    };
}

pub fn get_search_from_request(req: &HttpRequest) -> String {
    let params = get_params_from_request(req);

    return if params.s.is_some() {
        params.s.as_ref().unwrap().to_string()
    } else {
        "".into()
    };
}
