// Project: rio
// Author: Greg Folker

use::clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cmd.yaml");
    let matches = App::from(yaml).get_matches();

    let verbosity = matches.value_of("verbosity");

    println!("Running app with verbosity={:?}", verbosity);
}
