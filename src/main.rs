extern crate clap;
extern crate hubcaps;
extern crate reqwest;
extern crate select;
extern crate url;
extern crate futures;
extern crate tokio;

use hubcaps::{Credentials, Github, git::GetReferenceResponse::Exact, git::GetReferenceResponse::StartWith};
use select::document::Document;
use select::predicate::Name;
use url::Url;
use futures::{Future, Stream};

fn fetch_codes() -> impl Future<Item=(), Error=()> {
    let buffetti_url = "https://buffetti.it/shopping-bag/";
    let stanosas_url = "http://www.stanosas.it";

    document_from(buffetti_url)
        .join(document_from(stanosas_url))
        .map(|(buffetti_code, stanosas_code)| {
            // https://stackoverflow.com/questions/53119182/conditionally-chaining-a-rust-future-based-on-initial-future-result/53119404#53119404
            if buffetti_code == stanosas_code {
                println!("Both sites use code: {:?}", buffetti_code);
            } else {
                println!(
                    "Buffetti released a new code: {:?}",
                    buffetti_code
                    );
            }
        })
        .map_err(|error| { println!("{:?}", error) })
}

fn document_from(url: &str) -> impl Future<Item=String, Error=reqwest::Error> {
    reqwest::async::Client::new().get(url).send().and_then(|response| {
        response.into_body().concat2()
    }).map(|body| {
        let body = std::str::from_utf8(&body).unwrap();
        volantino_code_from(Document::from(body))
    })
}

fn volantino_code_from(document: Document) -> String {
    let iframe = document.find(Name("iframe")).next().unwrap();
    // println!("IFRAME: {:?}", iframe);
    let src = iframe.attr("src").unwrap();
    let src_url = Url::parse(&["data:", &src].concat()).unwrap();
    let (_param_key, param_value) = src_url.query_pairs().find(|(x,_y)| x == "d").unwrap();
    println!("SRC_URL: {:?}", param_value);

    return param_value.to_string()
}

// {
//     Ok(reference) => {
//         match reference {
//             Exact(reference) => {
//                 let current_branch_sha = reference.object.sha;
//                 println!("SHA {:#?}", current_branch_sha);
//             },
//             StartWith(_) => { panic!("There are many branches starting with heads/gh-pages") }
//         }
//     },
//     Err(err) => println!("err {:#?}", err),
// }
// fn get_branch_ref() -> impl Future<Item=hubcaps::git::GetReferenceResponse, Error=hubcaps::Error> {
//     let github = Github::new(
//         "my-cool-user-agent/0.1.0",
//         Credentials::Token(api_token.to_string()),
//     );
//     github
//         .repo("enricostano", "stanosas")
//         .git()
//         .reference("heads/gh-pages")
// }

// 1. create a new branch from gh-pages
// 1.a. get gh-pages branch's SHA
// 1.b. create a new branch pointing to the SHA from 1.a.
//      More details: https://gist.github.com/potherca/3964930

// 2. create a new content updating the file with the new code
//    https://developer.github.com/v3/repos/contents/
// 3. create a new PR
fn main() {
    tokio::run(fetch_codes());
}
