use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::JoyconData;
use crate::joycon_data_set::JoyconDataSet;

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
        use crate::schema::joycon_data;
        diesel::delete(joycon_data::table).execute(&mut self.conn)
    }

    pub fn create_new_joycon_entry(&mut self, entry: &JoyconData) -> QueryResult<usize>
    {
        use crate::schema::joycon_data;
        diesel::insert_into(joycon_data::table)
            .values(entry)
            .execute(&mut self.conn)
    }

    // adds an entire data set to the database.
    pub fn create_new_joycon_dataset(&mut self, entry: &JoyconDataSet) -> QueryResult<usize>
    {
        let dataset: Vec<JoyconData> = entry.into();
        use crate::schema::joycon_data;
        diesel::insert_into(joycon_data::table)
                .values(dataset)
                .execute(&mut self.conn)
    }

    pub fn get_new_training_number(&mut self, sym: &str) -> QueryResult<Option<i32>>
    {
        use crate::schema::joycon_data::dsl::*;
        return joycon_data
            .filter(symbol.eq(sym))
            .select(diesel::dsl::max(training_num))
            .first(&mut self.conn);
    }
}


