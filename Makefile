SHELL = /bin/sh

all: deps env clean build install manpage

deps:
	@echo Checking for dependencies...
ifeq ($(shell command -v fzf),)
	@echo "Please install fzf as it is required for some features."
endif

env:
	@echo Checking environment for Rust compiler...
ifeq ($(shell command -v cargo),)
	@echo "'cargo' is required for installation."
endif

clean:
	cargo clean

build:
	cargo build --release

install:
	@echo \(Unimplemented\) Installing...

manpage:
	@echo \(Unimplemented\) Installing manpage...
