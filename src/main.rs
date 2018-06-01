extern crate reqwest;
extern crate select;
extern crate github_rs;
extern crate serde_json;
use select::document::Document;
use select::predicate::Name;
use github_rs::client::{Executor, Github};
use serde_json::Value;

fn document_from(url: &str) -> Document {
    let client = reqwest::Client::new();
    let mut response = client.get(url).send().unwrap();
    let text = response.text().unwrap();

    Document::from(&*text)
}

fn volantino_code_from(url: &str) -> String {
    let document = document_from(url);
    let iframe = document.find(Name("iframe")).next().unwrap();
    let src = iframe.attr("src").unwrap();

    src.rsplit("/").next().unwrap().to_string()
}

fn main() {
    let buffetti_url = "https://buffetti.it/shopping-bag";
    let stanosas_url = "http://www.stanosas.it";
    let buffetti_volantino_code = volantino_code_from(buffetti_url);
    let stanosas_volantino_code = volantino_code_from(stanosas_url);

    // if buffetti_volantino_code != stanosas_volantino_code {
    //     println!("DIFFERENT")
    // } else {
    //     println!("Both sites use code: {:?}", buffetti_volantino_code)
    // }

    let client = Github::new("API TOKEN").unwrap();
    let me = client.get()
                   .user()
                   .execute::<Value>();
    println!("code: {:?}", me)
}
