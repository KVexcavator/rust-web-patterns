компиляция
rustc hello.rs
chmod +x hello.rs
выполнение
./hello
===
cargo run
cargo run --release
cargo doc --open
---
03-to-do/project
```to_do/core/src/main.rs
se std::env;

fn main(){
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
}
```
cargo run -p to-do-core one two three
cargo run -p to-do-core --release one two three
---
03-to-do/project
cargo add -p to-do-core clap
cargo add -p to-do-core clap --features derive