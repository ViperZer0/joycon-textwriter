extern crate tensorflow;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{JoyconData, NewJoyconData};

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_joycon_data_entry(conn: &mut SqliteConnection, symbol: &str, training_num: i32) -> JoyconData {
    use crate::schema::joycon_data;
    let new_entry = NewJoyconData::new(symbol, training_num);

    diesel::insert_into(joycon_data::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error adding new data!")
}

fn main() {
    use self::schema::joycon_data::dsl::*;

    let connection = &mut establish_connection();
    let results = joycon_data
        .limit(5)
        .load::<models::JoyconData>(connection)
        .expect("Error loading posts!");

    println!("Displaying {} entries", results.len());
    for data in results
    {
        println!("{}, {}", data.symbol, data.training_num);
    }
}
