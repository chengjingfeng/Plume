use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use std::collections::HashMap;
use bcrypt::{hash, DEFAULT_COST};

use db_conn::DbConn;
use models::user::*;
use models::instance::Instance;


#[get("/@/<name>")]
fn details(name: String) {

}

#[get("/users/new")]
fn new() -> Template {
    Template::render("users/new", HashMap::<String, i32>::new())
}

#[derive(FromForm)]
struct NewUserForm {
    username: String,
    email: String,
    password: String,
    password_confirmation: String
}

#[post("/users/new", data = "<data>")]
fn create(conn: DbConn, data: Form<NewUserForm>) -> Redirect {
    let inst = Instance::get_local(&*conn).unwrap();
    let form = data.get();

    if form.password == form.password_confirmation {
        User::insert(&*conn, NewUser {
            username: form.username.to_string(),
            display_name: form.username.to_string(),
            outbox_url: User::compute_outbox(form.username.to_string(), inst.public_domain.to_string()),
            inbox_url: User::compute_inbox(form.username.to_string(), inst.public_domain.to_string()),
            is_admin: !inst.has_admin(&*conn),
            summary: String::from(""),
            email: Some(form.email.to_string()),
            hashed_password: Some(hash(form.password.as_str(), DEFAULT_COST).unwrap()),
            instance_id: inst.id
        });
    }
    
    Redirect::to(format!("/@/{}", data.get().username).as_str())
}
