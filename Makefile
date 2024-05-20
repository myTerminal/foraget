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

crater-get:
	@echo "Setting up Crater for temporary use..."
	git clone https://github.com/crater-space/cli /tmp/crater-cli

deps:
	@echo "Checking for 'fzf'..."
ifneq ($(shell command -v fzf),)
	@echo "'fzf' found."
else
	@echo "Attempting to install 'fzf' using Crater..."
	/tmp/crater-cli/crater install fzf
endif

env:
	@echo "Looking for Rust compiler..."
ifneq ($(shell command -v cargo),)
	@echo "'cargo' found, build can continue."
else
	@echo "Attempting to install 'cargo' using Crater..."
	/tmp/crater-cli/crater install cargo
endif

crater-remove:
	@echo "Removing Crater..."
	rm -rf /tmp/crater-cli

req: crater-get deps env crater-remove

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
	mkdir -p $(MANPREFIX)/man1
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
