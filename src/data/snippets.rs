#[derive(Clone, Debug)]
pub struct Snippet {
    pub lang: &'static str,
    pub title: &'static str,
    pub code: &'static str,
}

pub fn snippet_for(name: &str) -> Option<Snippet> {
    match name {
        "Leptos" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Clicked " {count} " times"
        </button>
    }
}

fn main() { mount_to_body(App) }"# }),

        "Dioxus" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use dioxus::prelude::*;

fn App() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        button { onclick: move |_| count += 1, "Clicked {count} times" }
    }
}

fn main() { dioxus::launch(App); }"# }),

        "Yew" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let count = use_state(|| 0);
    let onclick = { let count = count.clone();
        Callback::from(move |_| count.set(*count + 1)) };
    html! { <button {onclick}>{ format!("Clicked {} times", *count) }</button> }
}

fn main() { yew::Renderer::<App>::new().render(); }"# }),

        "Sycamore" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        let count = create_signal(cx, 0);
        view! { cx,
            button(on:click=move |_| count.set(*count.get() + 1)) {
                "Clicked " (count.get()) " times"
            }
        }
    });
}"# }),

        "Axum" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "hello, world!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}"# }),

        "Actix Web" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder { "hello, world!" }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 3000))?.run().await
}"# }),

        "Rocket" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str { "hello, world!" }

#[launch]
fn rocket() -> _ { rocket::build().mount("/", routes![index]) }"# }),

        "Warp" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "hello, world!");
    warp::serve(hello).run(([0, 0, 0, 0], 3000)).await;
}"# }),

        "Poem" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use poem::{get, handler, listener::TcpListener, Route, Server};

#[handler] fn hello() -> &'static str { "hello, world!" }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}"# }),

        "Loco" => Some(Snippet { lang: "bash", title: "shell", code: r#"# Scaffold a full MVC app — models, controllers, migrations, workers
$ cargo install loco-cli
$ loco new -n blog
$ cd blog
$ cargo loco generate scaffold post title:string body:text
$ cargo loco start"# }),

        "Perseus" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(
        Template::build("index").view(|cx| view! { cx, h1 { "hello, world!" } })
    )
}"# }),

        "Tokio" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(work("A"), work("B"));
    println!("done: {a} / {b}");
}

async fn work(name: &str) -> &str {
    sleep(Duration::from_millis(100)).await;
    name
}"# }),

        "async-std" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use async_std::task;

#[async_std::main]
async fn main() {
    task::spawn(async { println!("hello from a task") }).await;
}"# }),

        "smol" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"fn main() {
    smol::block_on(async {
        let task = smol::spawn(async { 1 + 1 });
        println!("{}", task.await);
    });
}"# }),

        "SQLx" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    // query! verifies this against the live DB at compile time
    let rec = sqlx::query!("SELECT name FROM users WHERE id = $1", 1i64)
        .fetch_one(&pool).await?;
    println!("hello, {}", rec.name);
    Ok(())
}"# }),

        "Diesel" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use diesel::prelude::*;
use crate::schema::users::dsl::*;

fn main() {
    let mut conn = PgConnection::establish(&std::env::var("DATABASE_URL").unwrap()).unwrap();
    let names: Vec<String> = users.select(name).limit(5).load(&mut conn).unwrap();
    for n in names { println!("hello, {n}"); }
}"# }),

        "SeaORM" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use sea_orm::*;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("postgres://user:pass@localhost/db").await?;
    let users: Vec<user::Model> = user::Entity::find().all(&db).await?;
    for u in users { println!("hello, {}", u.name); }
    Ok(())
}"# }),

        "tonic" => Some(Snippet { lang: "rust", title: "src/server.rs", code: r#"use tonic::{transport::Server, Request, Response, Status};
use hello::{greeter_server::{Greeter, GreeterServer}, HelloReply, HelloRequest};

#[derive(Default)] pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, req: Request<HelloRequest>)
        -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply { message: format!("hello, {}", req.into_inner().name) }))
    }
}"# }),

        "async-graphql" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use async_graphql::*;

struct Query;

#[Object]
impl Query {
    async fn hello(&self, #[graphql(default = "world")] name: String) -> String {
        format!("hello, {name}")
    }
}

fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    println!("{}", schema.sdl());
}"# }),

        "utoipa" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use utoipa::{OpenApi, ToSchema};

#[derive(ToSchema)] struct User { id: i64, name: String }

#[utoipa::path(get, path = "/users/{id}", responses(
    (status = 200, body = User)
))]
async fn get_user() {}

#[derive(OpenApi)]
#[openapi(paths(get_user), components(schemas(User)))]
struct ApiDoc;

fn main() { println!("{}", ApiDoc::openapi().to_pretty_json().unwrap()); }"# }),

        "Askama" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")] // templates/hello.html: Hello, {{ name }}!
struct HelloTpl<'a> { name: &'a str }

fn main() {
    let html = HelloTpl { name: "world" }.render().unwrap();
    println!("{html}");
}"# }),

        "Maud" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use maud::{html, Markup};

fn page(name: &str) -> Markup {
    html! {
        h1 { "hello, " (name) "!" }
    }
}

fn main() { println!("{}", page("world").into_string()); }"# }),

        "Tera" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use tera::{Context, Tera};

fn main() {
    let tera = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", "world");
    println!("{}", tera.render("hello.html", &ctx).unwrap());
}"# }),

        "reqwest" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("{}", &body[..200]);
    Ok(())
}"# }),

        "serde" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User { id: u64, name: String }

fn main() -> serde_json::Result<()> {
    let u = User { id: 1, name: "ferris".into() };
    let json = serde_json::to_string(&u)?;
    let back: User = serde_json::from_str(&json)?;
    println!("{json}  ->  {back:?}");
    Ok(())
}"# }),

        "hyper" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use http_body_util::Full;
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            hyper::server::conn::http1::Builder::new()
                .serve_connection(TokioIo::new(stream), service_fn(|_req| async {
                    Ok::<_, hyper::Error>(hyper::Response::new(Full::new(Bytes::from("hello"))))
                })).await.ok();
        });
    }
}"# }),

        "tower" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use tower::{ServiceBuilder, ServiceExt, service_fn};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let svc = ServiceBuilder::new()
        .timeout(Duration::from_secs(1))
        .service(service_fn(|req: &str| async move { Ok::<_, ()>(format!("hello, {req}")) }));

    let reply = svc.oneshot("world").await.unwrap();
    println!("{reply}");
}"# }),

        "tracing" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use tracing::{info, instrument};

#[instrument]
fn greet(name: &str) {
    info!(target: "app", "hello, {name}");
}

fn main() {
    tracing_subscriber::fmt().init();
    greet("world");
}"# }),

        "anyhow" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("while reading config.toml")
}

fn main() -> Result<()> {
    let cfg = read_config()?;
    println!("{cfg}");
    Ok(())
}"# }),

        "thiserror" => Some(Snippet { lang: "rust", title: "src/error.rs", code: r#"use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("io: {0}")]      Io(#[from] std::io::Error),
    #[error("not found: {0}")] NotFound(String),
}

pub fn load(name: &str) -> Result<String, AppError> {
    std::fs::read_to_string(name).map_err(Into::into)
}"# }),

        "cargo" => Some(Snippet { lang: "bash", title: "shell", code: r#"$ cargo new hello
$ cd hello
$ cat src/main.rs
fn main() { println!("hello, world!"); }

$ cargo add serde --features derive
$ cargo build --release
$ cargo test"# }),

        "wasm-bindgen" => Some(Snippet { lang: "rust", title: "src/lib.rs", code: r#"use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("hello, {name}"));
}"# }),

        "trunk" => Some(Snippet { lang: "bash", title: "shell", code: r#"# index.html  ->  <link data-trunk rel="rust" />
$ trunk serve --open
# live reload, wasm-bindgen + wasm-opt, CSS/SCSS/Tailwind — all wired.
$ trunk build --release"# }),

        "clap" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use clap::Parser;

#[derive(Parser)]
#[command(name = "hello", version, about)]
struct Cli {
    /// who to greet
    #[arg(short, long, default_value = "world")]
    name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("hello, {}!", cli.name);
}"# }),

        _ => None,
    }
}
