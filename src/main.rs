use database::{DbFairing, Db};
use rocket::{get, launch, routes, figment::{Figment, providers::{Toml, Format}}, Config, State, serde::json::Json};
use surrealdb::sql::Value;

mod database;

#[get("/")]
async fn index(db: &State<Db>) -> Json<Value> {

    db.query("CREATE account SET name = 'John Doe', created_at = time::now();").await.unwrap();

    let query = db.query("SELECT * FROM account WHERE name = 'John Doe';").await.unwrap();

    Json(query.clone().into_iter().nth(0).unwrap())
}

#[launch]
async fn rocket() -> _ {
    let figment = Figment::from(Config::default())
      .merge(Toml::file("Rocket.toml").nested())
      .merge(Toml::file("App.toml").nested());

    rocket::custom(figment).mount("/", routes![index]).attach(DbFairing)
}
