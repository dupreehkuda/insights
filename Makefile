.PHONY: deploy
deploy:
	docker pull ghcr.io/dupreehkuda/insights-prod:latest
	docker kill insights
	docker rm insights
	docker run -d --restart=always --network="host" --env-file=".env" --name insights ghcr.io/dupreehkuda/insights-prod:latest

.PHONY: beforecommit
beforecommit:
	cargo fmt
	cargo test