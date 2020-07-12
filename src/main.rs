mod transaction_parsing;

use transaction_parsing::process_transactions_csv;

fn main() 
{

    let file_name = "resources/transactions.csv";
    process_transactions_csv(&file_name);

}
