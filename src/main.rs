use axum::{
    Json, Router, http::{StatusCode}, response::{IntoResponse, Html}, routing::{get, post}
};
use std::{net::SocketAddr};
use tower_http::{
    trace::TraceLayer,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text};
use dotenvy::dotenv;
use std::env;

#[derive(serde::Deserialize, serde::Serialize)]
struct NewUser {
    name: String,
    lastName: String
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Equation {
    left: i32,
    right: i32
}

#[derive(Debug, QueryableByName)]
struct RawUser {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    name: String,
    #[diesel(sql_type = Text)]
    email: String,
}

fn raw_select(conn: &mut PgConnection) -> QueryResult<Vec<RawUser>> {
    diesel::sql_query("SELECT id, name, email FROM users WHERE id = $1")
        .bind::<Integer, _>(1)
        .load(conn)
}

#[tokio::main]
async fn main() {
    let mut conn = establish_connection();
    println!("Connected to Postgres!");

    match raw_select(&mut conn) {
        Ok(users) => println!("Loaded users: {:?}", users),
        Err(err) => eprintln!("Failed to load users: {}", err),
    }
    
    let app = create_app();
    tokio::join!(serve(app, 3000));
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {}", database_url, e))
}

fn create_app() -> Router {
    Router::new()
        .route("/user/create", post(create_user))
        .route("/user/calculate", post(calculate))
        .route("/", get(index))
}

// Вшиваем HTML в экзешник почему он стал в два раза меньше с этим чем без ?
async fn index() -> impl IntoResponse {
    static INDEX_HTML: &str = include_str!("../assets/index.html");
    Html(INDEX_HTML)
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Stared listening on {}", addr);
    axum::serve(listener, app.layer(TraceLayer::new_for_http())).await;
}

async fn calculate(Json(equation): Json<Equation>) -> impl IntoResponse {
    let value = serde_json::json!({
        "sum" : (equation.left + equation.right)
    });
    println!("Equation is: {} + {} = {}", equation.left, equation.right, equation.left + equation.right);
    (StatusCode::ACCEPTED, Json(value))
}

async fn create_user(Json(newUser): Json<NewUser>) -> impl IntoResponse {
    if newUser.name.trim().is_empty() || newUser.lastName.is_empty() {
        println!("Something is empty.");
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error" : "One field is missing"})));
    }

    let value = serde_json::json!({
        "name": newUser.name,
        "lastName" : newUser.lastName
    });

    (StatusCode::ACCEPTED, Json(value))
}