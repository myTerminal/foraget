SHELL = /bin/sh

ifeq ($(PREFIX),)
    PREFIX := /usr/local
endif
MANPREFIX := $(PREFIX)/share/man

help:
	@echo "Use one of the following options:"
	@echo "'make req' - Checks depencencies and environment"
	@echo "'make install' - Installs foraget"
	@echo "'make uninstall' - Uninstalls foraget"

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
	install ./target/release/foraget $(PREFIX)/bin/

manpage:
	@echo "Creating manpage..."
	cp ./man/foraget.1 $(MANPREFIX)/man1/
	@echo "Manpage created!"

install: req clean build place manpage
	@echo "foraget is now installed!"

uninstall:
	rm $(PREFIX)/bin/foraget
	rm $(MANPREFIX)/man1/foraget.1
	@echo "Uninstallation was successful!"
