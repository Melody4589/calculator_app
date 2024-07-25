#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::{Serialize, Deserialize};
use rocket::form::Form;
use std::path::Path;

#[derive(FromForm, Debug)]
struct Calculation {
    expression: String,
}

#[derive(Serialize, Deserialize)]
struct Result {
    result: f64,
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[post("/calculate", data = "<calculation>")]
fn calculate(calculation: Form<Calculation>) -> String {
    let expr = &calculation.expression;
    let result = meval::eval_str(expr).unwrap_or(0.0);
    serde_json::to_string(&Result { result }).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, calculate])
        .mount("/static", rocket::fs::FileServer::from("static"))
}
