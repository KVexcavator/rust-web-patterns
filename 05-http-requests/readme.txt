компиляция
rustc hello.rs
chmod +x hello.rs
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
Программа, которая принимает заголовок и статус из командной строки, взаимодействует с файлом и редактирует его в зависимости от команды и данных из этого файла

cargo add -p to-do-core clap
cargo add -p to-do-core clap --features derive
cargo run -p to-do-core -- --help
cargo run -p to-do-core -- --first-name Barbambia --last-name Kergudu --age 34

cargo run --features json-file-storage

cargo run --features json-file-storage -- \
--title coding \
--status pending
cargo run --features json-file-storage -- \
--title washing \
--status done

(result in task.json)
===
