#[macro_use]
extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> String {
    "hello".to_string()
}

#[get("/ok")]
fn healthcheck() -> String {
    "ok".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, index, healthcheck])
}
