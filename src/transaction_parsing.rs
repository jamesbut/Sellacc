use csv::Reader;
use std::error::Error;
use std::fs::File;

pub fn process_transactions_csv(file_name: &str)
{
    println!("Reading transactions from file: {}", file_name);
    let mut transactions_reader = Reader::from_path(file_name).unwrap();

    let transactions_data = process_transactions_data(&mut transactions_reader);

}

fn process_transactions_data(transactions_reader: &mut csv::Reader<File>) -> Vec<Vec<String>>
{

    let mut data: Vec<Vec<String>> = Vec::new();

    //Move relevant data into 2d vector of strings
    for (i, result) in transactions_reader.records().enumerate() 
    {
        //Remove top row
        if i == 0 
        {
            continue;
        }

        let record = result.unwrap();

        let mut record_vec = Vec::new(); 

        //Only the credentials and transaction amount are required
        let credentials_str = record.get(2).unwrap();
        let amount_str = record.get(3).unwrap();
        
        record_vec.push(String::from(credentials_str));
        record_vec.push(String::from(amount_str));
        
        //If amount < 0 do not include in data
        //These are outgoing transactions and are not considered here
        let amount = record_vec[1].parse::<f64>().unwrap(); 
        if amount > 0. {
            data.push(record_vec);
        }

    }

    println!("{:#?}", data);
    println!("-------------------------------");

    //Retrieve code from credentials
    /*
    for record in data.iter()
    {
        let trans_code = parse_credentials_for_code(&record[0]);
    }
    */

    data

}


fn parse_credentials_for_code(credentials: &str) -> String
{
    //println!("{:?}", credentials);
    return credentials.to_string();
}

