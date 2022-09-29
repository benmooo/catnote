pub fn subject() -> String {
    "Cat note verifycation".to_owned()
}

pub fn template(code: &str) -> String {
    format!(
        r"
    verification code: {}
    ",
        code
    )
}
