#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;
/*
#[get("/")]
fn index() -> &'static str {
    "Index"
}
*/

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title : "Rocket Template",
        content : "Hello, world!",
    })
}

#[get("/about")]
fn about() -> &'static str {
    "About"
}

#[get("/")]
fn projects() -> &'static str {
    "Projects"
}

#[post("/")]
fn create_project() -> &'static str {
    "Creating Project..."
}

#[delete("/")]
fn delete_project() -> &'static str {
    "Delete Project..."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about])
     .mount("/project", routes![projects,create_project,delete_project])
     .mount("/public", FileServer::from("static"))
     .attach(Template::fairing())
}