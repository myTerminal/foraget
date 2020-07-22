SHELL = /bin/sh

help:
	@echo "\(Unimplemented\) Help"

deps:
	@echo "Checking for dependencies..."
ifeq ($(shell command -v fzf),)
	@echo "Please install fzf as it is required for some features."
else
	@echo "'fzf' found."
endif

env:
	@echo "Checking environment for Rust compiler..."
ifeq ($(shell command -v cargo),)
	@echo "'cargo' is required for installation."
else
	@echo "'cargo' found, build is possible."
endif

req: deps env

clean:
	@echo "Cleaning build directory..."
	cargo clean

build:
	@echo Building project...
	cargo build --release

place:
	@echo "\(Unimplemented\) Placing in 'PATH'..."

manpage:
	@echo "\(Unimplemented\) Installing manpage..."

install: req clean build place manpage
	@echo "\(Unimplemented\) Installing..."
