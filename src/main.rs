use database::{DbFairing, Db};
use rocket::{get, launch, routes, figment::{Figment, providers::{Toml, Format}}, Config, State};

mod database;

#[get("/")]
async fn index(db: &State<Db>) -> &'static str {

    let query = db.query("CREATE account SET name = 'Test', created_at = time::now();").await.unwrap();

    println!("query res: {:?}", query);

    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let figment = Figment::from(Config::default())
      .merge(Toml::file("App.toml").nested());

    rocket::custom(figment).mount("/", routes![index]).attach(DbFairing)
}
