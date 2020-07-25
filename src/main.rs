mod transaction_parsing;
mod tenninety_http;
mod t_data;

use transaction_parsing::retrieve_transactions_data;
use tenninety_http::login;
use tenninety_http::property_search;

fn main() 
{

    let file_name = "resources/transactions.csv";
    let transactions_data = retrieve_transactions_data(&file_name);

    println!("{:#?}", transactions_data);

    login();
    //property_search(&transactions_data);

}
