default: all

## all
all: test

## test
.PHONY: test
test:
	@echo "Running tests..."
	@set RUST_BACKTRACE=1 && cargo test -- --nocapture
