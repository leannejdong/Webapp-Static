#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;


use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket::Request;

#[derive(serde::Serialize)]
struct Message {
    user: &'static str,
    body: &'static str
}

#[derive(serde::Serialize)]
struct BoardContext {
    current_user: Option<String>,
    messages: Vec<Message>,
    parent: &'static str
}

#[derive(serde::Serialize)]
struct AboutContext {
    parent: &'static str
}


#[get("/")]
fn index() -> Redirect {
    Redirect::to("/user/anonymous")
}

#[get("/user/<username>")]
fn board(username: String) -> Template {
    Template::render("index", &BoardContext {
        current_user: Some(username),
        messages: vec![Message{user: "userA", body: "This is the first test message."},
                        Message{user: "userB", body: "This is the second test message."}],
        parent: "layout"
    })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", &AboutContext {
        parent: "layout"
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}


fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, board, about])
        .register(catchers![not_found])
        
}

fn main() {
    rocket().launch();
}