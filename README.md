# syn_api

##About

A demo project for the purposes of benchmarking a Rust web CRUD API, with a backing MongoDB database.

##Running the application

###Database
There is a simple `docker-compose` configuration included in the project that will run a MongoDB container to serve as a local test database. 

Open a terminal and run `docker-compose up` to run the container.

###Server
Using [cargo-watch](https://crates.io/crates/cargo-watch)  we are able to run our server locally with hot-reloading. Install cargo-watch as a global cargo dependency by running `cargo install cargo-watch`

Once installed, open a terminal and run `cargo watch -q -c -w src/ -x run` to reload actix_web when code changes are made in the `src` directory.

To run the server without hot-reloading, run `cargo run`.

##Web Framework

###actix-web

After doing some research into the Rust web framework ecosystem, it seems to me that [actix-web](https://actix.rs/docs/) is the most sensible option. It has a similar style to other frameworks in its class, with focus on simple and concise code that leverages macros to condense things like route handlers into a few lines. 

It wins out over frameworks like [Axum](https://docs.rs/axum/latest/axum/) and [Rocket](https://rocket.rs/) in all of the benchmarking tests I was able to find, with more requests per second, lower CPU and memory usage, and higher data transfer rates.

It has a few great selling points:

* Simplicity: with syntax similar to [Express](https://expressjs.com/), actix-web makes writing routes and middleware a breeze - especially when you consider the [Extractors](https://actix.rs/docs/extractors/) it provides that exposes request data to request handlers automatically

* Multi-threaded by default: actix-web automatically configures a number of worker threads equal to the number of physical cores of the host machine - this can be easily configured to suite the needs of the application

* Asynchronous: leveraging Rust [Futures](https://rust-lang.github.io/async-book/02_execution/02_future.html) and [async/await](https://rust-lang.github.io/async-book/03_async_await/01_chapter.html), actix-web recommends a pattern in which request handlers are completely non-blocking - allowing for worker threads that are awaiting some work (e.g. database operations) to begin handling new requests while they wait  

* Graceful shutdown default: when a critical failure occurs, actix-web allows active request handlers some time (default 30 seconds) to complete their requests before exiting the process 

Overall, it seems to have the best balance between maturity, durability, performance, and accessibility.

*- Luke*

##Dependencies

###mongodb
Standard MongoDB driver for Rust. Documentation can be found [here.](https://docs.rs/mongodb/latest/mongodb/)

Note: this driver uses connection pooling by default (default pool of 20 connections).
