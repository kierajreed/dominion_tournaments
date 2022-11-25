// Set up rocket features and crates
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket_contrib::templates::{Template, handlebars};
use rocket_contrib::serve::StaticFiles;
use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};
mod tourney_db;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    path: String,
    data: Option<String>,
    parent: &'static str,
}

fn render_page<S: Into<String>, T: Into<String>>(template: &str, title: S, path: T, data: Option<String>) -> Template {
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

fn eq_helper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    if let Some(first) = h.param(0) {
        if let Some(second) = h.param(1) {
            if *(&first.value().render().eq(&second.value().render())) {
                if let Some(value) = h.param(2) {
                    out.write(&value.value().render())?;
                }
            }
        }
    }

    Ok(())
}

fn build_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, calendar, players, tournaments])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .register(catchers![not_found])
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("ifeq", Box::new(eq_helper));
        }))
}

fn main() {
    tourney_db::init().err();

    build_rocket().launch();
}