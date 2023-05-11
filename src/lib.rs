use diesel::prelude::*;
use diesel::query_builder::{DeleteStatement, IncompleteInsertStatement};
use dotenvy::dotenv;
use std::env;
use self::models::{JoyconData, JoyconDataSet};

pub mod models;
pub mod schema;

pub struct DatabaseConnection { }

pub struct OpenDatabaseConnection {
    conn: SqliteConnection
}

impl DatabaseConnection
{
    pub fn new() -> OpenDatabaseConnection
    {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        OpenDatabaseConnection
        {
            conn
        }
    }
}

impl OpenDatabaseConnection{
    pub fn clear(&mut self) -> QueryResult<usize>
    {
        use self::schema::joycon_data;
        diesel::delete(joycon_data::table).execute(&mut self.conn)
    }

    pub fn create_new_joycon_entry(&mut self, entry: &JoyconData) -> QueryResult<usize>
    {
        use self::schema::joycon_data;
        diesel::insert_into(joycon_data::table)
            .values(entry)
            .execute(&mut self.conn)
    }

    // adds an entire data set to the database.
    pub fn create_new_joycon_dataset(&mut self, entry: &JoyconDataSet) -> QueryResult<usize>
    {
        let dataset: Vec<JoyconData> = entry.into();
        use self::schema::joycon_data;
        diesel::insert_into(joycon_data::table)
                .values(dataset)
                .execute(&mut self.conn)
    }
}
