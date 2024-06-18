#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    return "Hello, world!";
}

#[get("/help/<page>")]
fn help(page: &str) -> &str {
    return page;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/contact", routes![help])
}

