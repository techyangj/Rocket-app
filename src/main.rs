#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::{NewRustacean, Rustacean};
use repositories::RustaceanRepository;

use diesel_migrations::*;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::serde::json::{json, Json, Value};
use rocket::Build;
use rocket::Rocket;
use rocket_sync_db_pools::database;
use std::io::stdout;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
// curl 127.0.0.1:8000/rustaceans/3 -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans/<id>")]
async fn view_rustaceans(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    // json!({"id": id, "name": "John", "email": "john@example.com"})
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -X POST -H 'Content-type: application/json' -d '{"name": "Alice", "email": "Alice@example.com"}'
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn crate_rustaceans(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

// curl 127.0.0.1:8000/rustaceans/1 -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -X PUT -H 'Content-type: application/json' -d '{"name": "New_Jack", "email": "New_Jack@example.com"}'
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustaceans(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    // json!({"id": id, "name": "John", "email":"John@example.com"})
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
// curl 127.0.0.1:8000/rustaceans/3 -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -X DELETE
#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

// async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
//     DbConn::get_one(&rocket)
//         .await
//         .expect("Failed to get db connection")
//         .run(|c| match c.run_pending_migrations(MIGRATIONS) {
//             Ok(()) => Ok(rocket),
//             Err(e) => {
//                 println!("Failed to run database migrations: {:?}", e);
//                 Err(rocket)
//             }
//         })
//         .await;
// }

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;
    rocket
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustaceans,
                crate_rustaceans,
                update_rustaceans,
                delete_rustaceans
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Running DB migrations", run_db_migrations))
        .launch()
        .await;
}
