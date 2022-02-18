#[macro_use]
extern crate rocket;

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
    "helo wodl ‽".to_string()
}

#[get("/ok")]
fn healthcheck() -> String {
    "ok".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, index, hello_kube, healthcheck])
}
