use axum::Form;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(Form(_form): Form<FormData>) {
    println!("{} {}", _form.name, _form.email);
}
