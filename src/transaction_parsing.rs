use csv::Reader;
//use std::error::Error;
use std::fs::File;

//A struct that collects relevant transaction data
#[derive(Debug)]
pub struct Tdata 
{
    property_code: String,
    amount: f64,
}

pub fn retrieve_transactions_data(file_name: &str) -> Vec<Tdata>
{
    //Open transactions csv file
    println!("Reading transactions from file: {}", file_name);
    let mut transactions_reader = Reader::from_path(file_name).unwrap();

    //Parse transaction data
    let transactions_data = process_transactions_data(&mut transactions_reader);

    transactions_data

}

fn process_transactions_data(transactions_reader: &mut csv::Reader<File>) -> Vec<Tdata>
{

    let mut transaction_data: Vec<Tdata> = Vec::new();

    //Move relevant data into 2d vector of strings
    for (i, result) in transactions_reader.records().enumerate() 
    {
        //Remove top row
        if i == 0 
        {
            continue;
        }

        let record = result.unwrap();

        //Only the credentials and transaction amount are required
        let credentials_str = record.get(2).unwrap();
        let amount_str = record.get(3).unwrap();
        
        //If amount < 0 do not include in data
        //These are outgoing transactions and are not considered here
        let amount = amount_str.parse::<f64>().unwrap(); 
        if amount < 0. 
        {
            continue;
        }

        //Retrieve property code from credentials
        let property_code = parse_credentials_for_code(credentials_str);

        let transaction = Tdata 
        {
            property_code,
            amount,
        };

        transaction_data.push(transaction);

    }

    transaction_data
}

fn parse_credentials_for_code(credentials: &str) -> String
{
    //Split credentials string by comma delimiter
    let split_credentials: Vec<&str> = credentials.split(",").collect();

    //So far it seems that the second part of the string is the code
    //I will have to do error checking on this though
    let property_code = split_credentials[1].trim();

    property_code.to_string()
}

