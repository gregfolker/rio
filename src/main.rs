// Project: rio
// Author: Greg Folker

use::clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let verbosity = matches.value_of("verbosity").unwrap_or("off");

    println!("Running app with verbosity={}", verbosity);

    let outfile = matches.value_of("outfile").unwrap_or("STDOUT");

    println!("Running app with outfile={}", outfile);

    let runtime = matches.value_of("runtime").unwrap_or("0");

    println!("Running app with runtime={}", runtime);
}
