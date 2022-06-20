
#[post("/login")]
pub fn login() -> &'static str {
    "Login works!"
}

#[post("/register")]
pub fn register() -> &'static str {
    "Register works!"
}

#[get("/account/<acc>")]
pub fn get_account(acc: String) -> String {
    acc
}
