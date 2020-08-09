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

    /* Select particular letting */
    
    //Retrieve most recent letting from html
    let most_recent_let_href = parse_lettings_list(&lettings_list_html);
    println!("{:#?}", most_recent_let_href);
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
