#[macro_use]
extern crate rocket;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;
use rocket::serde::{Serialize, json::Json};


#[derive(Serialize)]
struct Rule {
    name: String,
    rule: String,
    extra: String,
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/kube")]
fn hello_kube() -> String {
    "helo kube".to_string()
}

#[get("/")]
fn index() -> String {
    "hellllllllllllo wodl ‽".to_string()
}

#[get("/rules")]
async fn rules_list(pool: &rocket::State<PgPool>) -> Json<Vec<Rule>> {
    sqlx::query_as!(Rule, "SELECT name, rule, extra FROM rules").fetch_all(&**pool).await.unwrap().into()
}

#[get("/rules/<name>")]
async fn rules_detail(name: String, pool: &rocket::State<PgPool>) -> Option<Json<Rule>> {
    let thing = Rule {
        name: "aaa".to_string(),
        rule: "aaa".to_string(),
        extra: "aaa".to_string(),
    };
    println!("{}", thing.name);
    Some(sqlx::query_as!(Rule, "SELECT name, rule, extra FROM rules WHERE name = $1", name).fetch_optional(&**pool).await.unwrap()?.into())
}

#[get("/ok")]
fn healthcheck() -> String {
    "ok".to_string()
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("missing `DATABASE_URL` env variable"))
        .await
        .expect("error connecting to the db");
    
    sqlx::migrate!()
        .run(&pool)
        .await.unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
            hello,
            index,
            hello_kube,
            healthcheck,
            rules_list,
            rules_detail,
            ])
        .manage(pool)
        .launch()
        .await
        .unwrap();
}
