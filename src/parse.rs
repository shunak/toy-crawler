use select::document::Document;
use select::predicate::Name;
use url::{Url,ParseError};
use serde::Serialize;

#[derive(Serialize,Debug)]
pub struct Article{
    pub id: u32,
    pub topic: String,
    pub url: String,
}

pub fn main() -> eyre::Result<Vec<String>> {

    let response = reqwest::blocking::get("https://newsdig.tbs.co.jp/list/latest")?;
    let base_url = response.url().clone();
    let body = response.text()?;
    let doc = Document::from(body.as_str());
    let mut json: Vec<String> = vec![];
    let possible_offset_from: Vec<u32> = vec![34,36]; // index 0 is in case the weather is typhoon etc.

    let mut range_to = 21;
    let mut current_pos_from = possible_offset_from[1];
    let mut current_pos_to = current_pos_from + range_to;

    // let offset_from: u32 = 32;
    // let offset_to: u32 = 53;
    let mut content: String = "".to_string();

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
        if idx <= current_pos_from || idx >= current_pos_to {
             continue;
        }

        match Url::parse(href) {

            Ok(url) => { println!("{}", url); },

            Err(ParseError::RelativeUrlWithoutBase) => {
                let url = base_url.join(href)?;
                // println!("{}", url);

                let doc2 = Document::from(reqwest::blocking::get(url.clone())?.text()?.as_str());

                let id = idx - current_pos_from;

                for title in doc2.find(Name("h1")).filter_map(|b| b.first_child()){

                    // println!("{}",idx);
                     // println!("ID:{} {:?} {}",id, title.html().replace("\u{3000}"," ").replace("&nbsp;",""), url.clone());

                     // Construct JSON format.
                    let data = Article {
                        id: id,
                        topic: title.html().replace("\u{3000}"," ").replace("&nbsp;",""),
                        url: url.clone().to_string(),
                    };

                    content = serde_json::to_string(&data).unwrap();

                    json.push(content);

                }

            },

            Err(e) => {println!("Error: {}",e)},

        }


    }

    Ok(json)
    // println!("body = {:?}", body);
    // Ok(content)
    // Ok(())
}

