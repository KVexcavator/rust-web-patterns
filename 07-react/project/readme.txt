чистить браузер
Ctrl + Shift + R
dev/prod serve_frontend_asset -> ingress/src/main.rs
===
Фронтенд React:
https://esbuild.github.io/getting-started/#install-esbuild
cd frontend
npm install --save-exact --save-dev esbuild
./node_modules/.bin/esbuild --version
npm install
npm run build
npm run serve
http://localhost:3000/
===
запустить фронт с rust-embed
cargo run -p ingress
запустить через скрипт
sh ingress/scripts/run_server.sh
локалхост:
http://localhost:8001/
===
Бекенд Rust:
запустить скрипт для тестирования:
sh scripts/basic_test.sh
---
urls:
http://localhost:8080/api/v1/get/all

http://localhost:8080/api/v1/get/{name}

curl -X POST http://127.0.0.1:8080/api/v1/create \
-H "Content-Type: application/json" \
-d '{"title": "writing", "status": "PENDING"}'

curl -X DELETE http://127.0.0.1:8080/api/v1/delete/writing