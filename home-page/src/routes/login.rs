use crate::db::Db;
use crate::model::{DbUser, Login};
use crate::routes::rocket_uri_macro_index;
use crate::tools::{hash_password, verify_password};
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};
use rocket::Route;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use uuid::Uuid;

#[post("/login", data = "<login>")]
pub async fn login(
    jar: &CookieJar<'_>,
    login: Form<Login<'_>>,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if let Ok(user) = sqlx::query_as::<_, DbUser>(
        r#"
        SELECT id, name, hashed_password, role, created
        FROM users
        WHERE name = ?1
        "#,
    )
    .bind(login.name)
    .fetch_one(&mut **db)
    .await
    {
        if verify_password(login.password, &user.hashed_password) {
            jar.add_private(("user_id", user.id.to_string()));
            Ok(Flash::success(
                Redirect::to(uri!(index)),
                "You logged in successfully",
            ))
        } else {
            Err(Flash::error(Redirect::to(uri!(index)), "Invalid password"))
        }
    } else {
        Err(Flash::error(Redirect::to(uri!(index)), "No such user"))
    }
}

#[post("/signup", data = "<login>")]
pub async fn signup(
    jar: &CookieJar<'_>,
    login: Form<Login<'_>>,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if let Ok(_user) = sqlx::query_as::<_, DbUser>(
        r#"
        SELECT id, name, hashed_password, role, created
        FROM users
        WHERE name = ?1
        "#,
    )
    .bind(login.name)
    .fetch_one(&mut **db)
    .await
    {
        Err(Flash::error(
            Redirect::to(uri!(index)),
            "That username has already been taken",
        ))
    } else {
        let uuid = Uuid::new_v4();

        if sqlx::query(
            r#"
            INSERT INTO users (id, name, hashed_password, role)
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(&uuid)
        .bind(login.name)
        .bind(hash_password(login.password))
        .bind("user")
        .execute(&mut **db)
        .await
        .is_ok()
        {
            jar.add_private(("user_id", uuid.to_string()));
            Ok(Flash::success(
                Redirect::to(uri!(index)),
                "You signed up successfully",
            ))
        } else {
            Err(Flash::error(Redirect::to(uri!(index)), "Database error :("))
        }
    }
}

pub fn routes() -> Vec<Route> {
    routes![login, signup]
}
