#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> String {
    format!("You have arrived at the root of the HAT API.")
}

fn rocket() -> rocket::Rocket {
    return rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![]);
} 

fn main() {
    rocket().launch();
}