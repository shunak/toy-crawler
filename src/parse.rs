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
    let offset_from: u32 = 33;
    let offset_to: u32 = 54;
    let mut content: String = "".to_string();

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
        if idx <= offset_from || idx >= offset_to {
             continue;
        }

        match Url::parse(href) {

            Ok(url) => { println!("{}", url); },

            Err(ParseError::RelativeUrlWithoutBase) => {
                let url = base_url.join(href)?;
                // println!("{}", url);

                let doc2 = Document::from(reqwest::blocking::get(url.clone())?.text()?.as_str());

                let id = idx - offset_from;

                for title in doc2.find(Name("h1")).filter_map(|b| b.first_child()){

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

