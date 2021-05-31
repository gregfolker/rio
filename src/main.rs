// Project: rio
// Author: Greg Folker

use::clap::{App, load_yaml};
use::std::thread;
use::std::time::Duration;

use::rio::ThreadPool;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let verbosity = matches.value_of("verbosity").unwrap_or("off");
    let outfile = matches.value_of("outfile").unwrap_or("STDOUT");
    let runtime: u64 = matches.value_of("runtime").unwrap_or("0").parse().expect("Expected a numerical value");
    let thread_count: usize = matches.value_of("num-threads").unwrap_or("1").parse().expect("Expected a numerical value");

    if verbosity != "off" {
        println!("Running app with verbosity={}", verbosity);
        println!("Running app with outfile={}", outfile);
        println!("Running app with runtime={}", runtime);
        println!("Running app with thread_count={}", thread_count);
     }

    let threads = ThreadPool::new(thread_count);

    println!("Waiting for {} thread(s) to complete in {} second(s)...", thread_count, runtime);
    thread::sleep(Duration::from_millis(runtime * 1000));
    println!("Done!");
}
