#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Rocket is a web framework for Rust!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
