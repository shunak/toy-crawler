use select::document::Document;
use select::predicate::Name;
use url::{Url,ParseError};


pub fn main() -> eyre::Result<Vec<String>> {

    let response = reqwest::blocking::get("https://newsdig.tbs.co.jp/list/latest")?;
    let base_url = response.url().clone();
    let body = response.text()?;
    let doc = Document::from(body.as_str());
    let mut json: Vec<String> = vec![];

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

                for title in doc2.find(Name("h1")).filter_map(|b| b.first_child()){

                        println!("No:{} {:?} {}",idx, title.html().replace("\u{3000}"," ").replace("&nbsp;",""), url.clone());
                        json.push(idx.to_string());
                        json.push(title.html().replace("\u{3000}"," ").replace("&nbsp;",""));
                        json.push(String::from(url.clone()));

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

