use csv::Reader;
use std::error::Error;
use std::fs::File;

fn process_transactions_reader(transactions_reader: &mut csv::Reader<File>) -> Vec<Vec<String>> {

    for result in transactions_reader.records() {

        let record = result.unwrap();
        println!("{:?}", record);

    }

    let data: Vec<Vec<String>> = Vec::new();

    data

}

//fn read_transactions_csv(file_name: &str) -> Result<(), Box<dyn Error>> {
fn read_transactions_csv(file_name: &str) {

    println!("Reading transactions from file: {}", file_name);

    let mut reader = Reader::from_path(file_name).unwrap();
    let data = process_transactions_reader(&mut reader);

}

fn main() {

    let file_name = "resources/transactions.csv";
    read_transactions_csv(&file_name);

}
