use axum::{Json, routing::*, Router, extract::Query};
use serde::{Deserialize, Serialize};
use axum::response::Html;
use maud::{html, Markup, DOCTYPE};

#[tokio::main(flavor = "current_thread")] //make axum single thread to match to bun
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/hello_html", get(hello_html));

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
pub struct HelloParams {
    name: String,
    num1: f64,
    num2: f64,
}

#[derive(Serialize)]
pub struct HelloResult {
    hello: String,
    sum: f64,
    mul: f64,
    sub:  f64,
    div: Option<f64>,
}

pub async fn hello( Query(query): Query<HelloParams>) -> Json<HelloResult>  {
    let result = HelloResult {
        hello: format!("hello {}", query.name),
        sum: query.num1 + query.num2,
        sub: query.num1 - query.num2,
        mul: query.num1 * query.num2,
        div: if query.num2 == 0. { None } else { Some(query.num1 / query.num2) }
    };

    Json(result)
}

pub async fn hello_html( Query(query): Query<HelloParams>) -> Html<String>  {

    let t = html!{
        div {
            div {
                div { "hello" }
                div {"sum"}
                div {"sub"}
                div {"mul"}
                div {"div"}
            }
            div {
                div {(query.name)}
                div {((query.num1 + query.num2).to_string())}
                div {((query.num1 - query.num2).to_string())}
                div {((query.num1 * query.num2).to_string())}
                div { (if query.num2 == 0. { "null".to_string() } else { (query.num1 / query.num2).to_string() } ) }
            }
        }
    };

    Html(t.into_string())
}