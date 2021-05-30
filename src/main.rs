// Project: rio
// Author: Greg Folker

use::clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let verbosity = matches.value_of("verbosity").unwrap_or("off");
    let outfile = matches.value_of("outfile").unwrap_or("STDOUT");
    let runtime = matches.value_of("runtime").unwrap_or("0");

    if verbosity != "off" {
        println!("Running app with verbosity={}", verbosity);
        println!("Running app with outfile={}", outfile);
        println!("Running app with runtime={}", runtime);
    }
}
