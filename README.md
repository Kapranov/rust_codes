# rust codes

## Downloading Installer

Welcome to Rust!

This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin
directory, located at:

```bash

  /home/kapranov/.cargo/bin
```

This path will then be added to your PATH environment variable by modifying the
profile file located at:

```bash
/home/kapranov/.profile
```

You can uninstall at any time with rustup self uninstall and these changes will
be reverted.

Current installation options:

*   default host triple: `x86_64-unknown-linux-gnu`
*     default toolchain: `stable`
*  modify PATH variable: `yes`

1. Proceed with installation (default)
2. Customize installation
3. Cancel installation

Rust is installed now. Great!

To get started you need Cargo's bin directory (`$HOME/.cargo/bin`) in your
`PATH` environment variable. Next time you log in this will be done automatically.

To configure your current shell run source `$HOME/.cargo/env`

## Setting `rustup default nightly` and back to stable

```sh
rustc --version # rustc 1.26.2 (594fb253c 2018-06-01)
rustup default nightly

rustc --version # rustc 1.28.0-nightly (86a8f1a63 2018-06-17)
rustup default stable

rustup show

# stable-x86_64-unknown-linux-gnu
# nightly-x86_64-unknown-linux-gnu (default)

rustup self uninstall

rustc --version
```

## The first Rocket application

1. `rustup default nightly`
2. `cargo new hello-rocket --bin`
3. `rustup update && cargo update`
4. edit `Cargo.toml`:

```rust
[dependencies]
rocket = "0.3.13"
rocket_codegen = "0.3.13"
```
5. edit `src/main.rs`:

```rust
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
    "Jurassic World: Fallen Kingdom is a 2018 American science fiction
adventure film"
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
```

6. Compile and run the program: `cargo run`

7. Visit `http://localhost:8000` to see your first Rocket application
   in action!

## Rust Examples


```rust
use std;

fn main(args: [str]) {
  std::io::println("hello world from " + args[0] + "!");
}
```

```rust
fn fac(n: int) -> int {
  let result = 1, i = 1;
  while i <= n {
    result *= i;
    i += 1;
  }
  ret result;
}
```

```rust
#[get("/data")]
fn load_from_database() -> redis::RedisResult<String> {
  let con = try!(client.get_connection());
  try!(con.get("mydata"))
}

fn main() {
  let client = redis::Client::open("redis://127.0.0.1/").unwrap();
  rocket::ignite().mount("/", routes![load_from_database]).launch();
}
```

```rust
use rocket::State;

struct MyConfig(String);

#[get("/")]
fn index(state: State<MyConfig>) -> String {
  format!("The config value is: {}", state.0)
}

#[get("/raw")]
fn raw_config_value<'r>(state: State<'r, MyConfig>) -> &'r str {
  state.inner().0.as_str()
}

fn main() {
  let config = MyConfig("user input".to_string());
  rocket::ignite()
    .mount("/", routes![index, raw_config_value])
    .manage(config)
    .launch()
}
```

```rust
lazy_static! {
  pub static ref DB_POOL: r2d2::Pool<ConnectionManager<PgConnection>> = {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool.");
    pool
  };
}

pub struct DB(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
  pub fn conn(&self) -> &PgConnection {
    &*self.0
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
  type Error = r2d2::GetTimeout;
  fn from_request(_: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    match DB_POOL.get() {
      Ok(conn) => Success(DB(conn)),
      Err(e) => Failure((Status::InternalServerError, e)),
    }
  }
}
```

## examples

1. 'hello' - hello example
2. 'gcd' - Euclid's algorithm
3. 'hello-rocket' - first Rocket application

[1]:  https://doc.rust-lang.org/stable/rust-by-example/hello/comment.html
[2]:  https://www.rust-lang.org/en-US/
[3]:  https://rocket.rs/
[4]:  http://ironframework.iu/
[5]:  https://aml3.github.io/RustTutorial/
[6]:  https://doc.rust-lang.org/stable/rust-by-example/
[7]:  https://stevedonovan.github.io/rust-gentle-intro/
[8]:  https://medium.com/learning-rust/rust-basics-e73304ab35c7
[9]:  https://github.com/Apress/beginning-rust
[10]: https://rurust.github.io/rust_book_ru/
[11]: http://words.steveklabnik.com/a-30-minute-introduction-to-rust
[12]: https://medium.com/sean3z/rest-api-node-vs-rust-c75aa8c96343
[13]: https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8
[14]: https://github.com/SergioBenitez/Rocket/tree/v0.3.13/examples/
[15]: http://siciarz.net/24-days-rust-cargo-and-cratesio/
[16]: http://siciarz.net/24-days-rust-conclusion-2016/
[17]: http://zsiciarz.github.io/24daysofrust/book/vol1/day1.html
[18]: https://rocket.rs/guide/overview/
[19]: http://siciarz.net/24-days-rust-cargo-and-cratesio/
[20]: https://github.com/ProgrammingRust/examples
[21]: https://github.com/SergioBenitez/Rocket

### 18 June 2018 by Oleg G.Kapranov
