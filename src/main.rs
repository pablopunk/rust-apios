#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate dotenv;
extern crate rocket;
extern crate bson;
extern crate mongodb;
#[macro_use] extern crate rocket_contrib;

use dotenv::dotenv;
use std::env;
use rocket_contrib::{Json};
use mongodb::{Client, ThreadedClient};
use mongodb::db::{ThreadedDatabase, DatabaseInner};
use std::sync::Arc;

fn get_db() -> Arc<DatabaseInner> {
   let db_host = env::var("DB_HOST").unwrap();
   let db_port = env::var("DB_PORT").unwrap().parse::<u16>().unwrap();
   let db_name = env::var("DB_NAME").unwrap();
   let client = Client::connect(&db_host, db_port)
      .ok().expect("Failed to connect to mongo");

   let db = client.db(&db_name);
   let db_user = env::var("DB_USER").unwrap();
   let db_pass = env::var("DB_PASS").unwrap();

   db.auth(&db_user, &db_pass)
      .ok().expect("Failed to authenticate client");

   db
}

#[get("/<resource>")]
fn resource_route(resource: String) -> Json {
   let db = get_db();
   let collection = db.collection(&resource);
   let cursor = collection.find(None, None).unwrap();
   let mut items = Vec::new();

   for res in cursor {
      items.push(res.ok())
   }

   Json(json!(items))
}

#[get("/")]
fn collections_route() -> Json {
   let db = get_db();
   let mut collections = db.collection_names(None).unwrap();
   collections.remove(0);

   Json(json!(collections))
}

fn main() {
   dotenv().ok();

   rocket::ignite()
      .mount("/", routes![resource_route, collections_route])
      .launch();
}
