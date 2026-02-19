// cargo run -p to-do-core
// cargo add -p to-do-core clap
// cargo run --features json-file-storage
mod api;
mod enums;
mod structs;
use api::basic_actions::create::create;

fn main() {
    let to_do_item = create("washing", enums::TaskStatus::PENDING);
    println!("{}", to_do_item);
}
