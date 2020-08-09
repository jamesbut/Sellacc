use reqwest::blocking::Client;
use crate::t_data::Tdata;
use crate::html_parser::*;

pub fn full_chain(transactions: &Vec<Tdata>)
{

    //Testing with HAR-082
    let transaction = &transactions[1];
    let ref_code = &transaction.property_code;


    let client = Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    
    login(&client);

    property_search(&client, &ref_code);

    let lettings_list_html = list_lettings(&client, &ref_code);

    //Retrieve most recent letting from html
    let most_recent_let_href = parse_lettings_list(&lettings_list_html);

    letting_select(&client, &most_recent_let_href);

    //Get receipts page because we need to parse the html to build the post request
    let receipts_html = letting_receipts(&client, &most_recent_let_href);

    //Parse receipts html for relevant information for receipt post request
    let wrk_key = most_recent_let_href.split('=').last().unwrap().to_string();
    let receipts_post_data = parse_receipts(&receipts_html, wrk_key);

}

fn login(client: &Client)
{

    let init_login_url = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp";

    let login_params = [("Id", "James"), 
                        ("Password", "tester"), 
                        ("login.x", "0"), 
                        ("login.y", "0")];

    let request = client.post(init_login_url)
        .form(&login_params)
        .build()
        .unwrap();

    println!("{:#?}\n", request);

    let response = client.execute(request).unwrap();

    println!("{:#?}", response);

}

fn property_search(client: &Client, ref_code: &String)
{

    let property_search_url = "https://hub1.10ninety.co.uk/lettings/admin/propertylist.asp";

    let prop_search_params = [("propsearchqualifier", "reference"), 
                              ("propsearch", ref_code), 
                              ("psearch.x", "0"), 
                              ("psearch.y", "0")];
    
    let prop_search_request = client.post(property_search_url)
        .form(&prop_search_params)
        .build()
        .unwrap();

    println!("{:#?}\n", prop_search_request);

    let prop_search_response = client.execute(prop_search_request)
        .unwrap();

    println!("{:#?}", prop_search_response);

}

fn list_lettings(client: &Client, ref_code: &String) -> String
{

    let lettings_list_url = "https://hub1.10ninety.co.uk/lettings/admin/lettingslist.asp";

    let lettings_search_request = client.get(lettings_list_url)
        .query(&[("Ref", ref_code)])
        .build()
        .unwrap();

    println!("{:#?}\n", lettings_search_request);

    let lettings_search_response = client.execute(lettings_search_request)
        .unwrap();

    println!("{:#?}\n", lettings_search_response);

    return lettings_search_response.text()
        .ok()
        .unwrap();

}

fn letting_select(client: &Client, query_string: &String)
{

    let letting_select_url = "https://hub1.10ninety.co.uk/lettings/admin/LettingsDetail.asp";

    let ref_query_val = query_string.split('=').last().unwrap();

    let letting_select_request = client.get(letting_select_url)
       .query(&[("Ref", ref_query_val)])
       .build()
       .unwrap();

    println!("{:#?}", letting_select_request);

    let letting_select_response = client.execute(letting_select_request)
        .unwrap();

    println!("{:#?}", letting_select_response);

}

fn letting_receipts(client: &Client, query_string: &String) -> String
{

    let letting_receipt_url = "https://hub1.10ninety.co.uk/lettings/admin/LettingsReceipts.asp";

    let ref_query_val = query_string.split('=').last().unwrap();

    let letting_receipt_request = client.get(letting_receipt_url)
       .query(&[("Ref", ref_query_val)])
       .build()
       .unwrap();

    println!("{:#?}", letting_receipt_request);

    let letting_receipt_response = client.execute(letting_receipt_request)
        .unwrap();

    println!("{:#?}", letting_receipt_response);

    return letting_receipt_response.text()
        .ok()
        .unwrap();
}

fn input_receipt(client: &Client)
{
    //I need to somehow build this huge post request
    //All the information is in the html though so I am going to have to do some 
    //serious parsing
}
