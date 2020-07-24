use reqwest::blocking::Client;
use reqwest::header;
use std::io::Read;
use std::process;
use crate::t_data::Tdata;

pub fn login()
{
    let google_url = "https://www.google.com";
    let tenninety_url = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp";
    let tenninety_redirect_url = "https://hub1.10ninety.co.uk/lettings/admin/Admin.Asp";

    let client = Client::new();

    //Build header 
    let mut headers = header::HeaderMap::new();

    build_login_header(&mut headers);

    /*
    let params = [("Id", "James"), 
                  ("Password", "tester"), 
                  ("login.x", "0"), 
                  ("login.y", "0")];
    */

    let mut request = client.get(tenninety_redirect_url)
        .headers(headers);

    println!("{:?}\n", request);

    let mut response = request.send().unwrap();

    println!("{:?}\n", response);

    let mut body = String::new();
    response.read_to_string(&mut body);

    println!("Body: \n{:?}", body);

}

pub fn property_search(transactions: &Vec<Tdata>)
{
    println!("Property search")
}

fn build_login_header(headers: &mut header::HeaderMap)
{

    let host = "hub1.10ninety.co.uk";
    headers.insert(header::HOST, header::HeaderValue::from_static(host));
    let connection = "keep-alive";
    headers.insert(header::CONNECTION, header::HeaderValue::from_static(connection));
    let cache_control = "max-age=0";
    headers.insert(header::CACHE_CONTROL, header::HeaderValue::from_static(cache_control));
    let upgrade_insecure_requests = "1";
    headers.insert(header::UPGRADE_INSECURE_REQUESTS, 
                   header::HeaderValue::from_static(upgrade_insecure_requests));
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 \
                      (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36";
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(user_agent));
    let accept = "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,\
                  */*;q=0.8,application/signed-exchange;v=b3;q=0.9";
    headers.insert(header::ACCEPT, header::HeaderValue::from_static(accept));
    let seqfetchsite_name = "seq-fetch-site";
    let seqfetchsite_val = "same-origin";
    headers.insert(header::HeaderName::from_static(seqfetchsite_name), 
                   header::HeaderValue::from_static(seqfetchsite_val));
    let seqfetchmode_name = "seq-fetch-mode";
    let seqfetchmode_val = "navigate";
    headers.insert(header::HeaderName::from_static(seqfetchmode_name), 
                   header::HeaderValue::from_static(seqfetchmode_val));
    let seqfetchuser_name = "seq-fetch-user";
    let seqfetchuser_val = "?1";
    headers.insert(header::HeaderName::from_static(seqfetchuser_name), 
                   header::HeaderValue::from_static(seqfetchuser_val));
    let seqfetchdest_name = "seq-fetch-dest";
    let seqfetchdest_val = "document";
    headers.insert(header::HeaderName::from_static(seqfetchdest_name), 
                   header::HeaderValue::from_static(seqfetchdest_val));
    let referer = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp?";
    headers.insert(header::REFERER, header::HeaderValue::from_static(referer));
    let accept_encoding = "gzip, deflate, br";
    headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static(accept_encoding));
    let accept_language = "en-GB,en-US;q=0.9,en;q=0.8";
    headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static(accept_language));
    let cookie = "ASPSESSIONIDCGQTTBRA=PNOOJOKCGGDEBFKNFDCOPDED; Tenant=; Letkey=; Landlord=; \
                  Company=Sellectlets; LetAct=; Position=Consultant; UserType=M; \
                  UserFullname=James+Butterworth; User=James; Connection=sellectlets; \
                  Test=test; Ref=; Password=tester";
    headers.insert(header::COOKIE, header::HeaderValue::from_static(cookie));

}
