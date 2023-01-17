use clap::{App, Arg};
use whois_rust::{WhoIs, WhoIsLookupOptions};

fn main() {
    let whois = WhoIs::from_path("./servers.json").unwrap();

    let matches = App::new("Whoopty")
    .version("1.0")
    .author("Harris Robin Kalash <hello@harris.dev>")
    .about("Whoopty is a CLI tool for looking up WHOIS information for domains")
    .arg(Arg::with_name("domain")
        .short('d')
        .long("domain")
        .value_name("DOMAIN")
        .help("Sets the domain to lookup")
        .takes_value(true))
    .get_matches();

    // get the value of the input argument
    let domain =
        matches.value_of("domain").unwrap_or("google.com");

    let result: String = whois
        .lookup(
            WhoIsLookupOptions::from_string(domain)
                .unwrap(),
        )
        .unwrap();

    println!("{:?}", result);
}
