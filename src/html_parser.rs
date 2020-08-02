use scraper::Html;

pub fn parse_lettings_list(lettings_list_html: &String)
{
    println!("Parsing");

    println!("{:#?}\n", lettings_list_html);

    let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

let document = Html::parse_document(html);
}
