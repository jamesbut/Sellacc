use reqwest::blocking::Client;
use reqwest::header;
use std::io::Read;
use std::process;

pub fn login()
{
    let google_url = "https://www.google.com";
    let tenninety_url = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp";

    let client = Client::new();

    //let mut response = client.get(tenninety_url).send().unwrap();
    
    /* Build header */
    let mut headers = header::HeaderMap::new();

    let host = "hub1.10ninety.co.uk";
    headers.insert(header::HOST, header::HeaderValue::from_static(host));
    let connection = "keep-alive";
    headers.insert(header::CONNECTION, header::HeaderValue::from_static(connection));
    let content_length = "44";
    headers.insert(header::CONTENT_LENGTH, header::HeaderValue::from_static(content_length));
    let cache_control = "max-age=0";
    headers.insert(header::CACHE_CONTROL, header::HeaderValue::from_static(cache_control));
    let upgrade_insecure_requests = "1";
    headers.insert(header::UPGRADE_INSECURE_REQUESTS, 
                   header::HeaderValue::from_static(upgrade_insecure_requests));
    let origin = "https://hub1.10ninety.co.uk";
    headers.insert(header::ORIGIN, header::HeaderValue::from_static(origin));
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 \
                      (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36";
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(user_agent));
    let accept = "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,\
                  */*;q=0.8,application/signed-exchange;v=b3;q=0.9";
    headers.insert(header::ACCEPT, header::HeaderValue::from_static(accept));
    //TODO: Come back to sec fetch ones
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
    let referer = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp?LoggedOut=true";
    headers.insert(header::REFERER, header::HeaderValue::from_static(referer));
    let accept_encoding = "gzip, deflate, br";
    headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static(accept_encoding));
    let accept_language = "en-GB,en-US;q=0.9,en;q=0.8";
    headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static(accept_language));
    let cookie = "Tenant=; Letkey=; Landlord=; LetAct=; Test=test; Ref=; Password=; Connection=; \
                  UserType=; Position=; Company=; User=;";
    headers.insert(header::COOKIE, header::HeaderValue::from_static(cookie));
    let userfullname_name = "userfullname";
    let userfullname_val = "ASPSESSIONIDAERQRCQA=LHOANHODAHHJMHCPKIFJKJKB";
    headers.insert(header::HeaderName::from_static(userfullname_name), 
                   header::HeaderValue::from_static(userfullname_val));

    //PROBLEM: I think the header names should have capitals because it uses http 1.1
    //but the library converts them all to lower case because this is a http 2.0 standard :/
    
    let params = [("Id", "James"), 
                  ("Password", "tester"), 
                  ("login.x", "0"), 
                  ("login.y", "0")];

    let mut request = client.post(tenninety_url)
        .headers(headers)
        .form(&params);

    println!("{:?}\n", request);

    //process::exit(0);

    let mut response = request.send().unwrap();
    
    /*
    let mut response = client.post(tenninety_url)
        //.body("Id=James&Password=tester&login.x=0&login.y=0")
        .headers(headers)
        .form(&params)
        .send()
        .unwrap();
    */
        

    println!("{:?}\n", response);

    let mut body = String::new();
    response.read_to_string(&mut body);

    println!("Body: \n{:?}", body);

}
