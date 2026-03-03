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