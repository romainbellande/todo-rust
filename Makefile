.DEFAULT_GOAL = help
## —— trivia ———————————————————————————————————————————————————————————————————————————————————————————————————————————
help: ## Outputs this help screen
	@grep -E '(^[a-zA-Z0-9_-]+:.*?##.*$$)|(^##)' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}{printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}' | sed -e 's/\[32m##/[33m/'

## —— certs ————————————————————————————————————————————————————————————————————————————————————————————————————————————
certs: ## Generate local certificates with mkcert
	mkcert -install && mkcert -cert-file certs/local-cert.pem -key-file certs/local-key.pem "todo.localhost" "*.todo.localhost"


migration_create:
	docker-compose exec api sqlx migrate add -r $(name)

migration_run:
	docker-compose exec api sqlx migrate run

migration_revert:
	docker-compose exec api sqlx migrate revert

.PHONY: help certs
