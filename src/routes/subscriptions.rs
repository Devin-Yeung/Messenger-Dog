use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use sqlx::{Pool, Postgres};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: Data<Pool<Postgres>>) -> HttpResponse {
    match sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref()) // need a mutable reference to the connection, or shared reference to the Pool
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
