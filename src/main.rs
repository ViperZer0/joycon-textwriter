extern crate tensorflow;

use joycon_typer::DatabaseConnection;
use joycon_typer::models::JoyconData;

fn main() {
    /*use self::schema::joycon_data::dsl::*;

    let connection = &mut establish_connection();

    create_joycon_data_entry(connection, "Test", 1);

    let results = joycon_data
        .limit(5)
        .load::<models::JoyconData>(connection)
        .expect("Error loading posts!");

    println!("Displaying {} entries", results.len());
    for data in results
    {
        println!("{}, {}", data.symbol, data.training_num);
    }*/


    let mut db_conn = DatabaseConnection::new();
    
    let testResult = JoyconData {
        symbol: String::from("A"),
        training_num: 1,
        sample_num: 1,
        time: None,
        gyro_x: None,
        gyro_y: None,
        gyro_z: None,
        accel_x: None,
        accel_y: None,
        accel_z: None,
    };

    db_conn.create_new_joycon_entry(&testResult);

    /*
    let results = joycon_data
        .limit(5)
        .load::<JoyconData>()
        .expect("Error loading posts!");

    println!("Displaying {} entries", results.len());
    for data in results
    {
        println!("{}, {}", data.symbol, data.training_num);
    }
    */
}
