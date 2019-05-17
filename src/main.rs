use hyper::Client;
use hyper::rt::{self, Future};
use clap::{Arg, App};
use regex::Regex;

fn main() {
    let arguments = App::new("Http Inspection and Fuzzing Interface")
                        .version("0.0.1")
                        .author("Jon Moroney jmoroney@hawaii.edu")
                        .after_help("Delimiters\n {pattern}")
                        .about("Fuzz some endpoints")
                        .arg(Arg::with_name("urls")
                               .short("u")
                               .long("urls")
                               .value_name("Urls")
                               .help("Urls to fuzz")
                               .min_values(1)
                               .required(true)
                               .takes_value(true)
                               .index(1))
                        .get_matches();

    let urls: Vec<_> = arguments.values_of("urls").unwrap().collect();
    let curly_regex = Regex::new(r"\{*\}").unwrap();
    for url in urls.iter() {
      println!("{:?}", url);
      println!("{:?}", curly_regex.is_match(url)); 
    }



    // rt::run(rt::lazy(|| {
    // let client = Client::new();

    // let uri = "http://httpbin.org/ip".parse().unwrap();

    // client
    //     .get(uri)
    //     .map(|res| {
    //         println!("Response: {}", res.status());
    //     })
    //     .map_err(|err| {
    //         println!("Error: {}", err);
    //     })
    // }));

    
}
