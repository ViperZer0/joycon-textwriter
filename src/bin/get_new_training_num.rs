use joycon_typer::database_connection::DatabaseConnection;

fn main() {
    let mut db_conn = DatabaseConnection::new();

    let num = db_conn.get_new_training_number("A").unwrap_or_default().unwrap_or_default();
    println!("{}", num);
}
