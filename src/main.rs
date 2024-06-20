#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    return "Hello, world!";
}

#[get("/sum/<num1>/<num2>")]
fn sum(num1: i128, num2: i128) -> String {
    return format!("result: {}", (num1 + num2));
}

#[get("/minus/<num1>/<num2>")]
fn minus(num1: i128, num2: i128) -> String {
    return format!("result: {}", (num1 - num2));
}

#[get("/multi/<num1>/<num2>")]
fn multi(num1: i128, num2: i128) -> String {
    return format!("result: {}", (num1 * num2));
}

#[get("/div/<num1>/<num2>")]
fn div(num1: i128, num2: i128) -> String {
    return format!("result: {}", (num1 / num2));
}

#[get("/pow/<num1>")]
fn pow(num1: i128) -> String {
    return format!("result: {}", (num1 * num1));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![sum])
        .mount("/", routes![minus])
        .mount("/", routes![multi])
        .mount("/", routes![div])
        .mount("/", routes![pow])
}

