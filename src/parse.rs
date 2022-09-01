use select::document::Document;
use select::predicate::Name;
use url::{Url,ParseError};


pub fn main() -> eyre::Result<Vec<String>> {

    let response = reqwest::blocking::get("https://newsdig.tbs.co.jp/list/latest")?;
    let base_url = response.url().clone();
    let body = response.text()?;
    let doc = Document::from(body.as_str());
    let mut json: Vec<String> = vec![];
    let offset: u32 = 33;

    // let url: &str = "";
    let mut idx: u32 = 0;
    // println!("{:?}", doc);
    // for href in doc.find(Name("a")){
    // for href in doc.find(Name("h1")).filter_map(|a| a.first_child()){
    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")){
        // println!("{:?}", href);
        // println!("{:?}", Url::parse(href));

        idx+=1;
        // println!("{}",idx);

        // Display Article only that is latest.
        if idx <= 33 || idx >= 54 {
             continue;
        }

        match Url::parse(href) {

            Ok(url) => { println!("{}", url); },

            Err(ParseError::RelativeUrlWithoutBase) => {
                let url = base_url.join(href)?;
                // println!("{}", url);

                let doc2 = Document::from(reqwest::blocking::get(url.clone())?.text()?.as_str());

                let id = idx - offset;
                for title in doc2.find(Name("h1")).filter_map(|b| b.first_child()){

                        println!("ID:{} {:?} {}",id, title.html().replace("\u{3000}"," ").replace("&nbsp;",""), url.clone());

                        // Construct JSON format.
                        let json_cnt1 = String::from("id") + &id.to_string() + ":{article title:" + &title.html().replace("\u{3000}"," ").replace("&nbsp;","").to_string() + ",url:" + &url.clone().to_string() + "}";

                        json.push(json_cnt1);

                }

            },

            Err(e) => {println!("Error: {}",e)},

        }

        // return json;



    }

    // println!("body = {:?}", body);

    Ok(json)
    // Ok(())
}

