SHELL = /bin/sh

ifeq ($(PREFIX),)
    PREFIX := /usr/local
endif
MANPREFIX := $(PREFIX)/share/man

help:
	@echo "Use one of the following options:"
	@echo "'make req' - Checks dependencies and environment"
	@echo "'make install' - Installs foraget"
	@echo "'make uninstall' - Uninstalls foraget"
	@echo "'make reinstall' - Reinstalls foraget"

deps:
	@echo "Checking for dependencies..."
ifeq ($(shell command -v fzf),)
	@echo "Please install 'fzf' as it is required for some features."
else
	@echo "'fzf' found."
endif

env:
	@echo "Checking environment for Rust compiler..."
ifeq ($(shell command -v cargo),)
	@echo "'cargo' is required for installation."
else
	@echo "'cargo' found, build can continue."
endif

req: deps env

clean:
	@echo "Cleaning build directory..."
	cargo clean
	@echo "Build directory cleaned"

build:
	@echo "Building project..."
	cargo build --release
	@echo "Build complete"

place:
	@echo "Installing binary..."
	sudo install ./target/release/foraget $(PREFIX)/bin/
	@echo "Binary installed"

manpage:
	@echo "Creating manpage..."
	sudo cp ./man/foraget.1 $(MANPREFIX)/man1/
	@echo "Manpage created"

install: req clean build place manpage
	@echo "foraget is now installed!"

uninstall:
	@echo "Uninstalling foraget..."
	sudo rm $(PREFIX)/bin/foraget
	sudo rm $(MANPREFIX)/man1/foraget.1
	@echo "Uninstallation was successful!"

reinstall: uninstall install
