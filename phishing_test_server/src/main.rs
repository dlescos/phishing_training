#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_db_pools::{Database, Connection};
use sqlx::{Executor, Row};


#[derive(Database)]
#[database("pg")]
struct Db(sqlx::PgPool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[post("/pawned/<organization>/<host>")]
async fn pawned(mut db: Connection<Db>, organization: &str, host: &str) -> String {
    let _= sqlx::query("insert into pawned (organization, host) values ($1, $2)")
        .bind(organization)
        .bind(host)
        .execute(&mut *db).await
        .ok();
    format!("pawned")
}

#[get("/pawned/<organization>")]
async fn get_pawned(mut db: Connection<Db>, organization: &str) -> String {
    let opt_rows = sqlx::query("select host from pawned where organization = $1")
        .bind(organization)
        .fetch_all(&mut *db).await
        .ok();

    if let Some(rows) = opt_rows {
        rows.iter().map(|r| r.get(0)).collect::<Vec<String>>().join(",")
    } else {
        "no value".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .attach(stage())
        .attach(Db::init())
        .attach(AdHoc::on_ignite("Create tables", |rocket| async move {
            // let _ = Box::pin(async move {
            if let Some(db) = Db::fetch(&rocket) {
                let conn = db.acquire().await;
                // create_tables(conn);
                let create_table = "CREATE TABLE IF NOT EXISTS pawned (
                    id serial PRIMARY KEY,
                    organization varchar(32) NOT NULL,
                    host varchar(32) NOT NULL,
                    timestamp timestamp DEFAULT now() NOT NULL
                );";
                conn.unwrap().execute(create_table).await.unwrap();

            }
            rocket
            // });
        }))
        .mount("/", routes![index, ping, pawned, get_pawned])
}
