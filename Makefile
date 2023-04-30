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
	docker compose --env-file .env.${mode} up -d
down:
	docker compose down --remove-orphans
ps:
	docker compose ps
logs:
	docker compose logs
logs-watch:
	docker compose logs --follow

# API
web:
	docker compose exec web bash

# Mysql
db:
	docker compose exec mysql bash

db-sql:
	docker compose exec mysql bash -c 'mysql -u $$MYSQL_USER -p$$MYSQL_PASSWORD $$MYSQL_DATABASE'
