# Set vars 
CC=cargo build
CFLAGS=--target x86_64-unknown-none

# Some colours
GREEN=\x1b[32;1m
RESET=\x1b[0m

# Set phony targets 
.PHONY: clean setup reset

# Setup for crosscompilation
setup:
	@printf "$(GREEN)Setting env...$(RESET)\n"
	rustup default nightly
	rustup target add x86_64-unknown-none

# Building os 
build:
	@printf "$(GREEN)Building...$(RESET)\n"
	$(CC) --release $(CFLAGS)

# Testing
test:
	@printf "$(GREEN)Building... (test)$(RESET)\n"
	$(CC) $(CFLAGS)

# Reset rustup default
reset:
	@printf "$(GREEN)Resetting env...$(RESET)\n"
	rustup default stable

clean:
	@printf "$(GREEN)Cleaning...$(RESET)\n"
	rm -rf target/release \
		target/x86_64-unknown-none/release/build \
		target/x86_64-unknown-none/release/deps \
		target/x86_64-unknown-none/release/examples \
		target/x86_64-unknown-none/release/incremental
