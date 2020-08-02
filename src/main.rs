mod transaction_parsing;
mod tenninety_http;
mod t_data;
mod html_parser;

use transaction_parsing::retrieve_transactions_data;
use tenninety_http::*;

fn main() 
{
    env_logger::init();

    let file_name = "resources/transactions.csv";
    let transactions_data = retrieve_transactions_data(&file_name);

    //println!("{:#?}", transactions_data);

    full_chain(&transactions_data);
    //login();
    //property_search(&transactions_data);
    //lettings_search();
    //lettings_detail_search();
    //receipts_search();
    //input_receipt();
    
}
