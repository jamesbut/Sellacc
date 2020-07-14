use reqwest::blocking::Client;
use std::io::Read;

pub fn login()
{
    let google_url = "https://www.google.com";
    let tenninety_url = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp";

    let client = Client::new();
    let mut response = client.get(tenninety_url).send().unwrap();
    println!("{:?}\n", response);

    let mut body = String::new();
    response.read_to_string(&mut body);

    println!("Body: \n{:?}", body);

}
