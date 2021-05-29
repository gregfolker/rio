// Project: rio
// Author: Greg Folker

use::clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cmd.yaml");
    let _matches = App::from(yaml).get_matches();
}
