use reqwest::blocking::Client;
use reqwest::blocking::RequestBuilder;
use reqwest::blocking::Response;
use reqwest::redirect::Policy;
use reqwest::header;
use std::io::Read;
use std::process;
use crate::t_data::Tdata;

pub fn full_chain(transactions: &Vec<Tdata>)
{

    //Testing with HAR-082
    let transaction = &transactions[1];
    let ref_code = &transaction.property_code;

    let init_login_url = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp";
    let client = Client::new();

    //let default_headers = build_default_headers();
    let default_headers = header::HeaderMap::new();

    let init_login_headers = build_init_login_headers(&default_headers);

    let login_params = [("Id", "James"), 
                        ("Password", "tester"), 
                        ("login.x", "0"), 
                        ("login.y", "0")];

    //let login_params_str = "Id=James&Password=tester&login.x=0&login.y=0";

    //let client = Client::new();
    
    let client = Client::builder()
        .cookie_store(true)
        //.redirect(Policy::none())
        //It looks like default headers are not inserted until execution
        //.default_headers(build_default_headers())
        .build()
        .unwrap();
    

    let request = client.post(init_login_url)
        .headers(init_login_headers)
        .form(&login_params)
        .build()
        .unwrap();

    println!("{:#?}\n", request);
    //println!("{:?}\n", request.body());

    let response = client.execute(request).unwrap();

    println!("{:#?}", response);
    //println!("{:?}", response.text());
    //let cookies = response.cookies();
    //for cookie in cookies
    //{
    //    println!("{:?}", cookie);
    //}

    return;

    /* Property search */
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

    let prop_search_response = client.execute(prop_search_request).unwrap();

    println!("{:#?}", prop_search_response);


    
}

pub fn login()
{
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

pub fn receipts_search()
{
    let receipts_search_url = 
        "https://hub1.10ninety.co.uk/lettings/admin/LettingsReceipts.asp?\
        Ref=HAR-082;10/05/2020";

    let client = Client::new();
    let default_headers = build_default_headers();

    //Build header
    let receipts_search_headers = build_receipts_search_headers(&default_headers);

    //Send request
    let request = client.get(receipts_search_url)
        .headers(receipts_search_headers);

    println!("Request: \n{:?}", request);

    let response = request.send().unwrap();

    display_response(response);
}

pub fn input_receipt()
{
    let input_receipt_url = "https://hub1.10ninety.co.uk/lettings/admin/LettingsReceipts.asp";

    let client = Client::new();
    let default_headers = build_default_headers();

    //let wrk_key = ;

    let body_string = "curPage=&WrkKey=HAR-082%3B10%2F05%2F2020&Date1=11%2F05%2F2020&Description1=+Rent+from+10%2F05%2F2020+to+09%2F06%2F2020&Amountdue1=500.00&Amount1=500.00&Receivedfrom1=t&Method1=F&Note1=&receivedby1=Neal&Duedate1=10%2F05%2F2020&Key1=39826&Upd1=0&Date2=10%2F06%2F2020&Description2=Rent+from+10%2F06%2F2020+to+09%2F07%2F2020&Amountdue2=500.00&Amount2=500.00&Receivedfrom2=t&Method2=F&Note2=&receivedby2=Neal&Duedate2=10%2F06%2F2020&Key2=40238&Upd2=0&Date3=10%2F07%2F2020&Description3=Rent+from+10%2F07%2F2020+to+09%2F08%2F2020&Amountdue3=500.00&Amount3=500.00&Receivedfrom3=t&Method3=F&Note3=&receivedby3=Neal&Duedate3=10%2F07%2F2020&Key3=40666&Upd3=0&Date4=29%2F07%2F2020&Description4=Rent+from+10%2F08%2F2020+to+09%2F09%2F2020&Amountdue4=500.00&Amount4=0.01&Receivedfrom4=t&Method4=F&Note4=&Receivedby4=James&Duedate4=10%2F08%2F2020&Key4=0&Upd4=1&MiscDate=&MiscDescription=&miscamountdue=&miscamount=&miscreceivedfrom=&miscmethod4=&miscnote=&miscreceivedby=&Tenant=Michala+Pigova&Letdate=10%2F05%2F2020&LetEnddate=09%2F05%2F2021&deposit=0&rent=500&fees=0&payfreq=m&Update1.x=44&Update1.y=6";
    let body_string_size = body_string.chars().count();
    println!("{:?}", body_string_size);

    let input_receipt_headers = build_input_receipt_headers(&default_headers, body_string_size);

    let request = client.post(input_receipt_url)
        .headers(input_receipt_headers)
        .body(body_string);

    println!("Request: \n{:?}", request);
    return;

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

fn build_init_login_headers(default_headers: &header::HeaderMap) -> header::HeaderMap
{
    let mut init_login_headers = default_headers.clone();

    
    //let referer = "https://hub1.10ninety.co.uk/sellectlets/admin-login.asp?";
    //init_login_headers.insert(header::REFERER, header::HeaderValue::from_static(referer));
    //Content length doesn't seem to be required either
    //let content_length = "44";
    //init_login_headers.insert(header::CONTENT_LENGTH,
    //                          header::HeaderValue::from_static(content_length));
    //I don't know whether origin should be part of default headers
    let origin = "https://hub1.10ninety.co.uk";
    init_login_headers.insert(header::ORIGIN,
                              header::HeaderValue::from_static(origin));
    /*
    let cookie = "ASPSESSIONIDCESRQCRA=JHLMCHFAKBIOPGLGAKIEDMOP";
    init_login_headers.insert(header::COOKIE,
                              header::HeaderValue::from_static(cookie));
    */

    init_login_headers
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

fn build_receipts_search_headers(default_headers: &header::HeaderMap) -> header::HeaderMap
{

    let mut receipts_search_headers = default_headers.clone();

    let referer = "https://hub1.10ninety.co.uk/lettings/admin/LettingsDetail.asp?\
                   Ref=HAR-082;10/05/2020";
    receipts_search_headers.insert(header::REFERER, 
                                   header::HeaderValue::from_static(referer));
    //This is the same cookie as the previous 2 - interesting to note
    let cookie = "ASPSESSIONIDAESQRDQB=JFKLPOCDKJDDFDDGAAPHAEFL; Company=Sellectlets; \
                  LetAct=; Position=Consultant; UserType=M; UserFullname=James+Butterworth; \
                  User=James; Connection=sellectlets; Test=test; Password=tester; \
                  Ref=HAR%2D082; sizeareafields=true; RMtenancyfees=true; \
                  RMtenancyfeesPropDet=true; Landlord=Lucinda+Mercer; Tenant=Michala+Pigova; \
                  ASPSESSIONIDAASQRDQB=KKPLPOCDHFJPGCHFFDJAOAEN; \
                  Letkey=HAR%2D082%3B10%2F05%2F2020";
    receipts_search_headers.insert(header::COOKIE, 
                                   header::HeaderValue::from_static(cookie));

    receipts_search_headers
}

fn build_input_receipt_headers(default_headers: &header::HeaderMap, 
                               body_size: usize) -> header::HeaderMap
{
    let mut input_receipt_headers = default_headers.clone();

    //Referer doesn't seem necessary
    //let referer = "https://hub1.10ninety.co.uk/lettings/admin/LettingsReceipts.asp?\
    //               Ref=HAR-082;10/05/2020";
    //input_receipt_headers.insert(header::REFERER, 
    //                             header::HeaderValue::from_static(referer));
    
    let cookie = "ASPSESSIONIDCESRQCRA=PIBGCHFAJKAFPEFOODDKBECM; Company=Sellectlets; \
                  LetAct=; Position=Consultant; UserType=M; UserFullname=James+Butterworth; \
                  User=James; Connection=sellectlets; Test=test; Password=tester; \
                  Ref=HAR%2D082; sizeareafields=true; RMtenancyfees=true; \
                  RMtenancyfeesPropDet=true; Landlord=Lucinda+Mercer; Tenant=Michala+Pigova; \
                  ASPSESSIONIDCASRQCRA=GKBGCHFACAHBMCLDJGEGCDCA; \
                  Letkey=HAR%2D082%3B10%2F05%2F2020; \
                  write%5Faccess%5Fset=true";
    input_receipt_headers.insert(header::COOKIE, 
                                 header::HeaderValue::from_static(cookie));

    //I am adding this here because I am adding body manually not via form
    let content_type = "application/x-www-form-urlencoded";
    input_receipt_headers.insert(header::CONTENT_TYPE,
                                 header::HeaderValue::from_static(content_type));
    let content_length = body_size.to_string();
    input_receipt_headers.insert(header::CONTENT_LENGTH,
                                 header::HeaderValue::from_str(&content_length).unwrap());

    input_receipt_headers
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
