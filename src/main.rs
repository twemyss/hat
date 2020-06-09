#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate rand;
extern crate ux;
#[macro_use] extern crate serde;

#[cfg(test)] mod tests;
mod codes;
mod optotypes;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::http::RawStr;
use rocket_contrib::serve::StaticFiles;
use crate::optotypes::{OptotypeArrangement};
use crate::codes::short::ShortCode;
use std::str::FromStr;

#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<String, String>::new())
}


#[get("/answers")]
fn code_form() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("code-form", context)
}

#[get("/answers?<code>")]
fn answer_display(code: &RawStr) -> Template {
    let parsed_code = match ShortCode::from_str(&code.to_string()) {
        Ok(code) => { code },
        Err(e) => { 
            let mut context = HashMap::<String, String>::new();
            context.insert("error".to_string(), format!("{}", e));
            return Template::render("code-form", context)
        }
    };
    let mut context = HashMap::<String, OptotypeArrangement>::new();
    context.insert("arrangement".to_string(), OptotypeArrangement::from(parsed_code));
    Template::render("answers", context)
}

fn rocket() -> rocket::Rocket {
    return rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, code_form, answer_display])
        .mount("/static", StaticFiles::from("static/"))
        .register(catchers![]);
} 

fn main() {
    rocket().launch();
}
