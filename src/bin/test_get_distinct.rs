use joycon_typer::database_connection::DatabaseConnection;
fn main()
{
    let mut db_conn = DatabaseConnection::new();
    let result = db_conn.get_all_symbol_types();
    println!("{:?}", result);
}
