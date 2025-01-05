use crate::db::Db;
use crate::model::{DbUser, User};
use crate::routes::rocket_uri_macro_index;
use rocket::response::{Flash, Redirect};
use rocket::Route;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/users")]
pub async fn users(user: User, mut db: Connection<Db>) -> Result<Template, Flash<Redirect>> {
    if let Ok(_user) = sqlx::query_as::<_, DbUser>(
        r#"
        SELECT id, name, hashed_password, role, created
        FROM users
        WHERE id = ?1
        AND role = ?2
        "#,
    )
    .bind(user.0)
    .bind("admin")
    .fetch_one(&mut **db)
    .await
    {
        rocket::info!("Here!");

        let users: Vec<DbUser> = sqlx::query_as("SELECT * FROM users")
            .fetch_all(&mut **db)
            .await
            .unwrap();

        Ok(Template::render("users", context! {users: users}))
    } else {
        Err(Flash::error(
            Redirect::to(uri!(index)),
            "Only admins can see a users list",
        ))
    }
}

pub fn routes() -> Vec<Route> {
    routes![users]
}
