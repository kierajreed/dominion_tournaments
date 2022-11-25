// Set up rocket features and crates
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    path: String,
    data: Option<String>,
    parent: &'static str,
}

fn render_page<S: Into<String>>(template: &str, title: S, path: S, data: Option<String>) -> Template {
    Template::render(format!("pages/{}", template), &TemplateContext {
        title: title.into(),
        path: path.into(),
        data: data,
        parent: "layouts/base",
    })
}

#[get("/")]
fn index() -> Template {
    render_page("index", "Dominion Tournaments", "/", None)
}

#[get("/about")]
fn about() -> Template {
    render_page("about", "About", "/about", None)
}

#[get("/calendar")]
fn calendar() -> Template {
    render_page("calendar", "Match Calendar", "/calendar", None)
}

#[get("/players")]
fn players() -> Template {
    render_page("players", "Players", "/players", None)
}

#[get("/tournaments")]
fn tournaments() -> Template {
    render_page("tournaments", "Tournaments", "/tournaments", None)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    
    Template::render("error/404", &map)
}

fn build_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, about, calendar, players, tournaments])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .register(catchers![not_found])
        .attach(Template::fairing())
}

fn main() {
    build_rocket().launch();
}