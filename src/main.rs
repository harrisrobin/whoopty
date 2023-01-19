use std::collections::HashMap;

use clap::{Arg, ArgAction, Command};
use whois_rust::{WhoIs, WhoIsLookupOptions};

fn main() {
    let whois = WhoIs::from_path("./servers.json").unwrap();

    let matches = Command::new("Whoopty")
        .version("1.0")
        .author("Harris Robin Kalash <hello@harris.dev>")
        .about("Whoopty is a CLI tool for looking up WHOIS information for domains")
        .arg(Arg::new("domain")
            .short('d')
            .long("domain")
            .value_name("DOMAIN")
            .action(ArgAction::Set)
            .help("Sets the domain to lookup")
        ).get_matches();

    // get the value of the input argument
    let domain = matches
        .get_one::<String>("domain")
        .expect("No domain provided");

    let result: String = whois
        .lookup(
            WhoIsLookupOptions::from_string(domain)
                .unwrap(),
        )
        .unwrap();

    let seperated = separate_sentences(&result);

    println!("{:#?}", seperated);
}

fn separate_sentences(
    text: &str,
) -> HashMap<String, String> {
    let mut sentences = HashMap::new();

    for line in text.lines() {
        let colon_index = match line.find(':') {
            Some(index) => index,
            None => 0,
        };

        if !line.is_empty() {
            let (before, after) =
                line.split_at(colon_index);
            sentences.insert(
                before.trim().to_string(),
                after[1..].trim().to_string(),
            );
        }
    }

    sentences
}
