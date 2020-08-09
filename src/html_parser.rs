use scraper::{Html, Selector};

//Parses lettings list for the href to the most recent letting
pub fn parse_lettings_list(lettings_list_html: &String) -> String
{
    let document = Html::parse_document(lettings_list_html);

    //There are no unique identifiers for the table itself but each element
    //in the table has either <td class=tablecell1> or <td class=tablecell2> 
    //which is unique to that table.
    let table_cell1_selector = Selector::parse(r#"td[class="tablecell1"]"#).unwrap();
    let table_cell2_selector = Selector::parse(r#"td[class="tablecell2"]"#).unwrap();
    let href_selector = Selector::parse("a").unwrap();

    let mut lettings_refs: Vec<String> = Vec::new(); 

    for table_cell in document.select(&table_cell1_selector)
    {
        if let Some(href_link) = table_cell.select(&href_selector).next() {
            if let Some(href_val) = href_link.value().attr("href") {
                lettings_refs.push(href_val.to_string());
            }
        }
    }

    for table_cell in document.select(&table_cell2_selector)
    {
        if let Some(href_link) = table_cell.select(&href_selector).next() {
            if let Some(href_val) = href_link.value().attr("href") {
                lettings_refs.push(href_val.to_string());
            }
        }
    }

    let most_recent_ref = calculate_most_recent_ref(&lettings_refs);
    most_recent_ref
}

fn calculate_most_recent_ref(lettings_refs: &Vec<String>) -> String
{
    let mut most_recent_ref = &lettings_refs[0];

    //Calculate most recent date
    for (i, let_ref) in lettings_refs.iter().enumerate()
    {
        let most_recent_date = most_recent_ref.split(';').last().unwrap();
        let date = let_ref.split(';').last().unwrap();
        if calculate_most_recent_date(date, most_recent_date) {
            most_recent_ref = &lettings_refs[i];
        }
    }

    most_recent_ref.to_string()
}

//Returns true if date1 is more recent than date2
fn calculate_most_recent_date(date1: &str, date2: &str) -> bool
{
    let d1_year = date1.split('/').nth(2);
    let d2_year = date2.split('/').nth(2);
    let d1_month = date1.split('/').nth(1);
    let d2_month = date2.split('/').nth(1);
    let d1_day = date1.split('/').nth(0);
    let d2_day = date2.split('/').nth(0);

    if d1_year > d2_year {
        return true;
    } else if d2_year > d1_year {
        return false;
    }

    if d1_month > d2_month {
        return true;
    } else if d2_month > d1_month {
        return false;
    }

    if d1_day > d2_day {
        return true;
    } else if d2_day > d1_day {
        return false;
    }

    true
}