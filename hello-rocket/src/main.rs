#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket::http::RawStr;

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Rocket is a web framework for Rust!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, Page!"
}

#[get("/world")]
fn world() -> &'static str {
    "World, Page!"
}

#[get("/hello/world")]
fn hello_world() -> &'static str {
    "World, page of the home!"
}

#[get("/welcome/<name>/<age>/<cool>")]
fn welcome(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/forward/<name>/<age>/<cool>")]
fn forward(name: String, age: u8, cool: bool) -> String {
    format!("You're a cool {},your age is {} year old, {}!", cool, age, name)
}

#[get("/proba/<name>")]
fn proba(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

mod other {
    #[get("/jurassic")]
    pub fn jurassic() -> &'static str {
        "Jurassic World is a 2015 American science fiction adventure film"
    }

    #[get("/jurassic/world")]
    pub fn world() -> &'static str {
        "Jurassic World: Fallen Kingdom is a 2018 American science fiction adventure film"
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
               index,
               hello,
               world,
               hello_world,
               welcome,
               proba,
               forward,
               other::jurassic,
               other::world
        ]).launch();
}
