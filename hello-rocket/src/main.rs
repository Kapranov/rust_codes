#![feature(plugin)]
#![plugin(rocket_codegen)]

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
               other::jurassic,
               other::world
        ]).launch();
}
