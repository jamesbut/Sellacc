use reqwest::blocking::Client;
use reqwest::blocking::RequestBuilder;
use reqwest::blocking::Response;
use reqwest::header;
use std::io::Read;
use std::process;
use crate::t_data::Tdata;

pub fn login()
{
    let google_url = "https://www.google.com";
    let tenninety_url = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp";
    let tenninety_login_url = "https://hub1.10ninety.co.uk/lettings/admin/Admin.Asp";

    //Put default headers in client I don't know whether adding more headers overrides
    //the default ones
    //I might actually be able to use this cookie store but I will come back to it

    let client = Client::new();

    let default_headers = build_default_headers();

    //Build header 
    let login_headers = build_login_headers(&default_headers);

    /*
    let params = [("Id", "James"), 
                  ("Password", "tester"), 
                  ("login.x", "0"), 
                  ("login.y", "0")];
    */

    let mut request = client.get(tenninety_login_url)
        .headers(login_headers);

    println!("{:?}\n", request);

    let mut response = request.send().unwrap();

    println!("{:?}\n", response);

    //let mut body = String::new();
    //response.read_to_string(&mut body);
    let mut body = response.text();

    println!("Body: \n{:?}", body);

}

pub fn property_search(transactions: &Vec<Tdata>)
{

    let property_search_url = "https://hub1.10ninety.co.uk/lettings/admin/PropDetail.asp?\
                               id=HAR-082";

    let client = Client::new();
    let default_headers = build_default_headers();

    //Build header 
    let property_search_headers = build_property_search_headers(&default_headers);

    //Send request
    let request = client.get(property_search_url)
        .headers(property_search_headers);

    println!("Request: \n{:?}", request);

    let response = request.send().unwrap();

    display_response(response);

}

pub fn lettings_search()
{
    let lettings_search_url = "https://hub1.10ninety.co.uk/lettings/admin/lettingslist.asp?\
                               Ref=HAR-082";

    let client = Client::new();
    let default_headers = build_default_headers();

    //Build header
    let lettings_search_headers = build_lettings_search_headers(&default_headers);

    //Send request
    let request = client.get(lettings_search_url)
        .headers(lettings_search_headers);

    println!("Request: \n{:?}", request);

    let response = request.send().unwrap();

    display_response(response);

}

pub fn lettings_detail_search()
{
    let lettings_detail_search_url = 
        "https://hub1.10ninety.co.uk/lettings/admin/LettingsDetail.asp?Ref=HAR-082;10/05/2020";

    let client = Client::new();
    let default_headers = build_default_headers();

    //Build header
    let lettings_detail_search_headers = 
        build_lettings_detail_search_headers(&default_headers);

    //Send request
    let request = client.get(lettings_detail_search_url)
        .headers(lettings_detail_search_headers);

    println!("Request: \n{:?}", request);

    let response = request.send().unwrap();

    display_response(response);

}


fn build_default_headers() -> header::HeaderMap
{
    let mut default_headers = header::HeaderMap::new();

    let accept = "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,\
                  */*;q=0.8,application/signed-exchange;v=b3;q=0.9";
    default_headers.insert(header::ACCEPT, header::HeaderValue::from_static(accept));
    let accept_encoding = "gzip, deflate, br";
    default_headers.insert(header::ACCEPT_ENCODING, 
                           header::HeaderValue::from_static(accept_encoding));
    let seqfetchsite_name = "seq-fetch-site";
    let seqfetchsite_val = "same-origin";
    default_headers.insert(header::HeaderName::from_static(seqfetchsite_name), 
                           header::HeaderValue::from_static(seqfetchsite_val));
    let seqfetchmode_name = "seq-fetch-mode";
    let seqfetchmode_val = "navigate";
    default_headers.insert(header::HeaderName::from_static(seqfetchmode_name), 
                           header::HeaderValue::from_static(seqfetchmode_val));
    let seqfetchuser_name = "seq-fetch-user";
    let seqfetchuser_val = "?1";
    default_headers.insert(header::HeaderName::from_static(seqfetchuser_name), 
                           header::HeaderValue::from_static(seqfetchuser_val));
    let seqfetchdest_name = "seq-fetch-dest";
    let seqfetchdest_val = "document";
    default_headers.insert(header::HeaderName::from_static(seqfetchdest_name), 
                           header::HeaderValue::from_static(seqfetchdest_val));
    let upgrade_insecure_requests = "1";
    default_headers.insert(header::UPGRADE_INSECURE_REQUESTS, 
                           header::HeaderValue::from_static(upgrade_insecure_requests));
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 \
                      (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36";
    default_headers.insert(header::USER_AGENT, header::HeaderValue::from_static(user_agent));
    let connection = "keep-alive";
    default_headers.insert(header::CONNECTION, header::HeaderValue::from_static(connection));
    let cache_control = "max-age=0";
    default_headers.insert(header::CACHE_CONTROL, 
                           header::HeaderValue::from_static(cache_control));
    let accept_language = "en-GB,en-US;q=0.9,en;q=0.8";
    default_headers.insert(header::ACCEPT_LANGUAGE, 
                           header::HeaderValue::from_static(accept_language));
    let host = "hub1.10ninety.co.uk";
    default_headers.insert(header::HOST, header::HeaderValue::from_static(host));

    default_headers

}

fn build_property_search_headers(default_headers: &header::HeaderMap) -> header::HeaderMap
{
    let mut property_search_headers = default_headers.clone();

    let referer = "https://hub1.10ninety.co.uk/lettings/admin/Admin.Asp";  
    property_search_headers.insert(header::REFERER, header::HeaderValue::from_static(referer));
    let cookie = "ASPSESSIONIDCGQTTBRA=PNOOJOKCGGDEBFKNFDCOPDED; Tenant=; Letkey=; Landlord=; \
                  Company=Sellectlets; LetAct=; Position=Consultant; UserType=M; \
                  UserFullname=James+Butterworth; User=James; Connection=sellectlets; \
                  Test=test; Ref=; Password=tester";
    property_search_headers.insert(header::COOKIE, header::HeaderValue::from_static(cookie));

    property_search_headers
}

fn build_login_headers(default_headers: &header::HeaderMap) -> header::HeaderMap
{

    let mut login_headers = default_headers.clone();

    let referer = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp?";
    login_headers.insert(header::REFERER, header::HeaderValue::from_static(referer));
    let cookie = "ASPSESSIONIDCGQTTBRA=PNOOJOKCGGDEBFKNFDCOPDED; Tenant=; Letkey=; Landlord=; \
                  Company=Sellectlets; LetAct=; Position=Consultant; UserType=M; \
                  UserFullname=James+Butterworth; User=James; Connection=sellectlets; \
                  Test=test; Ref=; Password=tester";
    login_headers.insert(header::COOKIE, header::HeaderValue::from_static(cookie));

    login_headers

}

fn build_lettings_search_headers(default_headers: &header::HeaderMap) -> header::HeaderMap
{

    let mut lettings_search_headers = default_headers.clone();

    let referer = "https://hub1.10ninety.co.uk/lettings/admin/propdetail.asp?Id=HAR-082";
    lettings_search_headers.insert(header::REFERER, header::HeaderValue::from_static(referer));
    let cookie = "ASPSESSIONIDAESQRDQB=JFKLPOCDKJDDFDDGAAPHAEFL; Company=Sellectlets; \
                  LetAct=; Position=Consultant; UserType=M; UserFullname=James+Butterworth; \
                  User=James; Connection=sellectlets; Test=test; Password=tester; \
                  Ref=HAR%2D082; sizeareafields=true; RMtenancyfees=true; \
                  RMtenancyfeesPropDet=true; Landlord=Lucinda+Mercer; Tenant=Michala+Pigova; \
                  ASPSESSIONIDAASQRDQB=KKPLPOCDHFJPGCHFFDJAOAEN; \
                  Letkey=HAR%2D082%3B10%2F05%2F2020";
    lettings_search_headers.insert(header::COOKIE, header::HeaderValue::from_static(cookie));

    lettings_search_headers

}


fn build_lettings_detail_search_headers(default_headers: &header::HeaderMap) 
    -> header::HeaderMap
{

    let mut lettings_detail_search_headers = default_headers.clone();

    let referer = "https://hub1.10ninety.co.uk/lettings/admin/lettingslist.asp?Ref=HAR-082";
    lettings_detail_search_headers.insert(header::REFERER, 
                                          header::HeaderValue::from_static(referer));
    //I think this cookie is the same as the one above
    let cookie = "ASPSESSIONIDAESQRDQB=JFKLPOCDKJDDFDDGAAPHAEFL; Company=Sellectlets; \
                  LetAct=; Position=Consultant; UserType=M; UserFullname=James+Butterworth; \
                  User=James; Connection=sellectlets; Test=test; Password=tester; \
                  Ref=HAR%2D082; sizeareafields=true; RMtenancyfees=true; \
                  RMtenancyfeesPropDet=true; Landlord=Lucinda+Mercer; Tenant=Michala+Pigova; \
                  ASPSESSIONIDAASQRDQB=KKPLPOCDHFJPGCHFFDJAOAEN; \
                  Letkey=HAR%2D082%3B10%2F05%2F2020";
    lettings_detail_search_headers.insert(header::COOKIE, 
                                          header::HeaderValue::from_static(cookie));

    lettings_detail_search_headers

}

fn display_response(response: Response)
{
    println!("{:?}\n", response);

    //let mut body = String::new();
    //response.read_to_string(&mut body);
    //let body = response.text();

    //println!("Body: \n{:?}", body);
    println!("Body: \n{:?}", response.text());
}
