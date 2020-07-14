SHELL = /bin/sh

all: deps env clean build install manpage
	@echo \(Unimplemented\) Default...

deps:
	@echo \(Unimplemented\) Getting dependencies...

env:
	@echo \(Unimplemented\) Preparing environment...

clean:
	@echo \(Unimplemented\) Cleaning...

build:
	@echo \(Unimplemented\) Building for current platform...

install:
	@echo \(Unimplemented\) Installing...

manpage:
	@echo \(Unimplemented\) Installing manpage...

ifeq ($(shell command -v fzf),)
	@echo "Please install fzf as it is required for some features."
endif
