# Setting
build:
	docker compose build --force-rm
build-nocache:
	docker compose build --no-cache --force-rm
up:
	docker compose --env-file .env.${mode} up
up-d:
	docker compose --env-file .env.${mode} up -d
restart:
	docker compose down --remove-orphans
	docker compose build --force-rm
	docker compose --env-file .env.${mode} up
down:
	docker compose down --remove-orphans
ps:
	docker compose ps
logs:
	docker compose logs
logs-watch:
	docker compose logs --follow

# Frontend Build
elm:
	cd api/frontend && elm make src/Main.elm --output public/main.js --optimize

# API
web:
	docker compose exec web bash

# Postgres
db:
	docker compose exec db bash

db-sql:
	docker compose exec db bash -c 'psql -U $$POSTGRES_USER -d $$POSTGRES_DB'
