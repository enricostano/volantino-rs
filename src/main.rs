extern crate clap;
extern crate hubcaps;
extern crate reqwest;
extern crate select;
extern crate tokio;

use clap::App;
use hubcaps::{Credentials, Github, git::GetReferenceResponse::Exact, git::GetReferenceResponse::StartWith};
use select::document::Document;
use select::predicate::Name;
use tokio::runtime::Runtime;

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

    if buffetti_volantino_code != volantino_code_from(stanosas_url) {
        println!("Both sites use code: {:?}", buffetti_volantino_code)
    } else {
        println!(
            "Buffetti released a new code: {:?}",
            buffetti_volantino_code
        );

        let github = Github::new(
            "my-cool-user-agent/0.1.0",
            Credentials::Token(api_token.to_string()),
        );
        let mut rt = Runtime::new().unwrap();

        // 1. create a new branch from gh-pages
        // 1.a. get gh-pages branch's SHA
        match rt.block_on(
            github
                .repo("enricostano", "stanosas")
                .git()
                .reference("heads/gh-pages")
        ) {
            Ok(reference) => {
                match reference {
                    Exact(reference) => { let current_branch_sha = reference.object.sha; },
                    StartWith(_) => { panic!("There are many branches starting with heads/gh-pages") }
                }
                println!("SHA {:#?}", current_branch_sha);
            },
            Err(err) => println!("err {:#?}", err),
        }

        // 1.b. create a new branch pointing to the SHA from 1.b.
        //      More details: https://gist.github.com/potherca/3964930

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
