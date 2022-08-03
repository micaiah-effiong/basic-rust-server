// use rocket::serde::{Serialize, json::Json};
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello Micaiah"
}

#[get("/home")]
fn home()->&'static str {
    "Welcome to home"
}

// #[derive(Serialize)]
// #[serde(crate = "rocket::serde")]
struct User {
    first_name: String
}

impl User {
    fn new(name:&str) -> Self {
        User { first_name: String::from(name) }
    }
}

#[get("/home/<id>")]
fn profile(id:&str)-> String {
    // let user = User::new("Micaiah"); //{ first_name: "Micaiah" }
    // Json(user)

    String::from(id)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index], )
        .mount("/",routes![home])
        .mount("/", routes![profile])
}