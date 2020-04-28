use clap::{load_yaml, App};
use header_generator::HeaderGenerator;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("{:#?}", matches);

    HeaderGenerator::default().run();
}
