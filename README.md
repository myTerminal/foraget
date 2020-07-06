# foraget

A simple universal package manager for Unix-like systems [in-progress]

## Background

Many of us move between operating systems often and need to remember commands for respective package managers. When you're on a Debian based system you might need `sudo apt install emacs`, if you're on Fedora you'd run `sudo dnf install emacs` while if you're on an Arch-based system, you'll run `sudo pacman -S emacs`. Learning about different package managers and their commands can sometimes be fun but may get difficult to remember.

*foraget* provides a simple, concise, easy to remember command-set to work with respective package managers available on the current platform to be able to search for, install, uninstall or run packages without having to remember commands for individual package managers. This means regardless of your setup, you'll always need to run `foraget install emacs` and *foraget* will do the rest for you.

### Why the name 'foraget'?

I wanted to name it something that is easy to spell as well as remember. Getting inspiration from names of real package managers that often end with 'get' and realizing that this one is nothing but a package that forages over output from other package managers, I came up with **fora(ge)t**.

## Installation

### As a binary crate using Cargo

If you already have [Cargo](https://github.com/rust-lang/cargo) installed, *foraget* can be installed directly from [crates.io](https://crates.io) using the below command:

    cargo install foraget

### As an application package

#### Compile from source

    # Clone project to local
    git clone https://github.com/myTerminal/foraget.git

    # Install with `make`
    make install

## How to Use

*foraget* can help you work with package managers without remembering commands for each of them.

### Enabling additional package sources (Not implemented)

*foraget* can help you set up additional package sources and package managers for your current platform with a single command.

    foraget init

Once the process is complete, it'll list down the additional enable software sources.

### Searching for a package

Searching for a package across package managers and sources has never been easier. Just use the `search` command and provide a search term and *foraget* will find for packages with similar names across all available package sources.

    foraget search emacs

### Installing a package

Installing packages is almost as simple as searching for them.

    foraget install emacs

As a part of the installation, *foraget* searches across all sources and provides a list of results to choose from. The selected package is then installed from the source that contains it.

### Uninstalling a package (Not implemented)

You would have probably used *foraget* to install a package and may not be sure of how to uninstall it. *foraget* can help you with uninstallation as well.

    foraget uninstall emacs

*foraget* figures out the package manager or source through which the package could be uninstalled.

## External Dependency

* [fzf](https://github.com/junegunn/fzf)

## To-do

* Implement uninstallation of a package
