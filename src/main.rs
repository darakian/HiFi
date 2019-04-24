use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use clap::{Arg, App};

fn main() {
    let arguments = App::new("Http Inspection and Fuzzing Interface")
                        .version("0.0.1")
                        .author("Jon Moroney jmoroney@hawaii.edu")
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

    println!("{:?}", arguments.values_of("urls").unwrap());




    rt::run(rt::lazy(|| {
    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse().unwrap();

    client
        .get(uri)
        .map(|res| {
            println!("Response: {}", res.status());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
    }));
}
