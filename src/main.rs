use csv::Reader;
use std::error::Error;
use std::fs::File;

fn read_transactions_csv(file_name: &str) -> Result<(), Box<dyn Error>> {

    println!("Reading transactions from file: {}", file_name);

    let mut reader = Reader::from_path(file_name)?;
    for result in reader.records() {

        let record = result?;
        println!("{:?}", record);

    }

    Ok(())

}

fn main() {

    let current_dir = std::env::current_exe().unwrap().to_str().unwrap().to_owned();
    let file_name = current_dir + "/../../../resources/transactions.csv";
    let mut file = File::open("foo.txt");
    //read_transactions_csv(&file_name);

}
