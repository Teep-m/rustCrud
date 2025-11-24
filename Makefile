.PHONY: up down build logs clean-db help iseed seed

help:
	@echo "使用可能なコマンド:"
	@echo "  make up       - コンテナをビルドして起動します（マイグレーション・シーダーも自動実行）"
	@echo "  make down     - コンテナを停止・削除します"
	@echo "  make build    - コンテナを再ビルドします"
	@echo "  make logs     - ログを表示します"
	@echo "  make clean-db - DBのデータを削除してリセットします（次回起動時に初期化）"
	@echo "  make iseed TABLE=tablename - 指定したテーブルのデータをSeeder(SQL)として逆生成します"
	@echo "  make seed FILE=filename    - 指定したSeederファイルを実行します（例: make seed FILE=seed_todos.sql）"

up:
	docker-compose up -d --build

down:
	docker-compose down

build:
	docker-compose build

logs:
	docker-compose logs -f

clean-db:
	docker-compose down -v

iseed:
	@if [ -z "$(TABLE)" ]; then echo "TABLE変数を指定してください (例: make iseed TABLE=todos)"; exit 1; fi
	docker-compose exec backend cargo run --bin iseed -- --table $(TABLE)
	docker cp $$(docker-compose ps -q backend):/app/migrations/seed_$(TABLE).sql ./backend/migrations/seed_$(TABLE).sql
	@echo "Seeder generated and copied to backend/migrations/seed_$(TABLE).sql"

seed:
	@if [ -z "$(FILE)" ]; then echo "FILE変数を指定してください (例: make seed FILE=seed_todos.sql)"; exit 1; fi
	docker-compose exec -T db psql -U user -d todo_db -f - < ./backend/migrations/$(FILE)
	@echo "Seeder executed: $(FILE)"
