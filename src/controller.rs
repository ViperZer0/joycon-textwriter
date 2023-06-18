use crate::database_connection::DatabaseConnection;
use crate::recorder::Recorder;
use crate::joycon_data_set::JoyconDataSet;
use std::io;
use std::io::Write;

pub struct Controller
{
}

impl Controller
{
    pub fn record_new_sample()
    {
        let symbol = Self::get_symbol();
        let sample_num = DatabaseConnection::new().get_new_training_number(&symbol);
        let sample_num = sample_num.expect("Something went wrong querying the database").unwrap_or_default() + 1;
        let sample = Recorder::new().get_sample().expect("Something went wrong with the recorder!");
        let dataset = JoyconDataSet
        {
            symbol: symbol,
            training_num: sample_num,
            data_points: sample
        };
        println!("{}", dataset);
        if Self::confirmation("Save this sample? [Y/n]:", true)
        {
            let result = DatabaseConnection::new().create_new_joycon_dataset(&dataset);
            if result.is_ok()
            {
                println!("Saved the sample to the database!");
            }
            else
            {
                println!("Something went wrong!");
            }
        }
    }

    fn get_symbol() -> String
    {
        let mut symbol = String::new();
        loop
        {
            println!("Enter the symbol to train: ");
            if io::stdin().read_line(&mut symbol).is_err()
            {
                println!("Something went wrong! Try again!");
                continue;
            }
            return symbol.trim().into()
        }
    }

    fn confirmation(prompt: &str, default: bool) -> bool
    {
        loop
        {
            let mut answer = String::new();
            print!("{}", prompt);
            std::io::stdout().flush().unwrap();
            if io::stdin().read_line(&mut answer).is_err()
            {
                println!("Something went wrong! Try again!");
                continue;
            }
            println!("Answer: {}", answer);
            if answer.trim().is_empty()
            {
                return default;
            }
            match answer.to_lowercase().trim()
            {
                "y" => return true,
                "n" => return false,
                _ => println!("Not a recognized option. Try again."),
            }
        }
    }
}
