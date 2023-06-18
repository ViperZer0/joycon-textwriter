use joycon_typer::database_connection::DatabaseConnection;

fn main()
{
    DatabaseConnection::new().clear_all_data().unwrap();
}

