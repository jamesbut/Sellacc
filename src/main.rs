mod transaction_parsing;
mod tenninety_http;

use transaction_parsing::retrieve_transactions_data;
use tenninety_http::login;

fn main() 
{

    let file_name = "resources/transactions.csv";
    let transactions_data = retrieve_transactions_data(&file_name);

    //println!("{:#?}", transactions_data);

    login();

}
