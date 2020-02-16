
FUNCTION_NAME := transactionsapi

start:
	cargo run --bin local

deploy:
	npx serverless deploy --verbose

remove:
	npx serverless remove -f $(FUNCTION_NAME) --verbose

test:
	cargo test

lint:
	cargo clippy

logs:
	npx serverless logs -f ${FUNCTION_NAME}

clean:
	@echo target directory will be removed, therein-by expect longer fresh compilation times.
	rm -r target || true

.PHONY: start deploy invoke remove test lint logs install