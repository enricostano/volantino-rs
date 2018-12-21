extern crate clap;
extern crate reqwest;
extern crate select;
#[macro_use]
extern crate serde_json;

use clap::{App};
use select::document::Document;
use select::predicate::Name;
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
    // println!("IFRAME: {:?}", iframe);
    let src = iframe.attr("src").unwrap();

    src.rsplit("/").next().unwrap().to_string()
}

fn main() {
//    let matches = App::new("volantino")
//        .version("0.1")
//        .get_matches();

    let buffetti_url = "https://buffetti.it/shopping-bag/";
    let stanosas_url = "http://www.stanosas.it";
    let buffetti_volantino_code = volantino_code_from(buffetti_url);

    if buffetti_volantino_code == volantino_code_from(stanosas_url) {
        println!("Both sites use code: {:?}", buffetti_volantino_code)
    } else {
        println!("Buffetti released a new code: {:?}", buffetti_volantino_code);

        //let api_token = "254c827df7507efd78d96d2fa98392e24e47962c";
        //let client = Github::new(api_token).unwrap();
        // 1. create a new branch from master

        // 2. create a new content updating the file with the new code
        //    https://developer.github.com/v3/repos/contents/
        // 3. create a new PR
//        let body = json!(
//            {
//                "title": "hola",
//                "body": "The code for the Volantino is changed. The new code is"
//            }
//            );
//        let me = client.get()
//            .repos()
//            .owner("enricostano")
//            .repo("stanosas")
//            .git()
//            .refs()
//            .execute::<Value>();
//        println!("code: {:?}", me)
    }
}
