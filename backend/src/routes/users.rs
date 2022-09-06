use rocket::post;
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::serde::{ Serialize, Deserialize, json::Json };
use regex::Regex;

#[derive(FromForm, Serialize, Deserialize)]
pub struct RegisterData {
    username: String,
    email: String,
    password: String
}

#[post("/register", format = "json", data = "<input>")]
pub async fn register(input: Json<RegisterData>) -> Result<Json<RegisterData>, Status> {
    // TODO: Password regex
    
    let username_regex = Regex::new(r"^[a-zA-Z0-9]{4,12}$").unwrap();
    // let password_regex = Regex::new(r"^(?=.*[A-Z])(?=.*[!@#$&*])(?=.*[0-9])(?=.*[a-z]).{8,16}$").unwrap();
    let email_regex = Regex::new(r"^\w+([.-]?\w+)*@\w+([.-]?\w+)*(\.\w{2,3})+$").unwrap();

    if !username_regex.is_match(&input.username) {
        return Err(Status::NotAcceptable);
    }

    if !email_regex.is_match(&input.email) {
        return Err(Status::NotAcceptable);
    }

    // if !password_regex.is_match(&input.password) {
    //     return Err(Status::NotAcceptable);
    // }

    Ok(Json(RegisterData {
        username: input.username.clone(),
        email: input.email.clone(),
        password: input.password.clone()
    }))
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct LoginData {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginDataResult {
    username: String,
    token: String
}

#[post("/login", format = "json", data = "<input>")]
pub async fn login(input: Json<LoginData>) -> Result<Json<LoginDataResult>, Status> {
    let username_regex = Regex::new(r"^[a-zA-Z0-9]{4,12}$").unwrap();
    let password_regex = Regex::new(r"^(?=.*[A-Z])(?=.*[!@#$&*])(?=.*[0-9])(?=.*[a-z]).{8,16}$").unwrap();

    if !username_regex.is_match(&input.username) {
        return Err(Status::NotAcceptable);
    }

    if !password_regex.is_match(&input.password) {
        return Err(Status::NotAcceptable);
    }

    Ok(Json(LoginDataResult {
        username: input.username.clone(),
        token: String::from("token")
    }))
}
