use clap::{Arg, App};
use std::collections::HashSet;
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
    let curly_regex = Regex::new(r"\{(.*?)\}").unwrap();
    let mut patterns: HashSet<(String, Option<Vec<String>>)> = HashSet::new();
    for url in urls.iter() {
      println!("url: {:?}", url);
      println!("url removed:{:?}", curly_regex.replace_all(url, ""));
      for cap in curly_regex.captures_iter(url) {
      Regex::new(&regex::escape(&cap[0])).unwrap();   
      println!("\tcap: {:?}", cap);
        for token in cap.iter(){
          println!("\t\ttoken: {:?}", token);
          //patterns.insert(token);
        }
        println!("\tcap[0]:{:?}", &cap[0]);
        patterns.insert((String::from(&cap[0]), None));
      }
      //patterns.push(curly_regex.captures(url));
    }

    println!("Captures");
    for element in patterns.iter(){
      println!("{:?}", element.0);
      let tmp = Regex::new(&regex::escape(&element.0)).unwrap();
      println!("{:?}", tmp);
      println!("is match? {:?}", tmp.is_match(&element.0));
    }
    //Sketch//
    //Parse urls for pattern points
    //Ensure that each pattern is valid
    //Generate input to patterns on worker threads with some random seed
    //Send generated data from worker threads to main thread
    //Async make http request


}
