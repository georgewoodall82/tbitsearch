# tbitsearch
A terminal bitsearch.to frontend.

# Installation

### Arch Linux
Just install the AUR package, here is an example with yay:
`yay -S tbitsearch-git`

Or with paru:
`paru -S tbitsearch-git`

### Other Distros
You can download the latest binary from the [release section](https://github.com/georgewoodall82/tbitsearch/releases) and add it to a directory in PATH, here is an example that should work on most systems:
`sudo cp [binary name] /usr/bin/tbitsearch`
Or, [build it yourself.](#building-1)

# Building
### Dependencies
The only main dependency this needs to build is cargo, which is included with rust, but some more may be needed. Here is an example command that installs all of the needed dependencies on arch linux:
`sudo pacman -S rust git gcc-libs openssl glibc --needed`
### Building
Just git clone this repo, cd to the directory and run cargo build. Here is a script that will do all of that for you:
```
git clone https://github.com/georgewoodall82/tbitsearch
cd tbitsearch
cargo bulid --release
```
The binary will be created in the `tbitsearch/target/release` folder

# Command line options
Just run `tbitsearch --help`, but if you don't want to here is the output:
```
Usage: tbitsearch [OPTIONS] <QUERY>

Arguments:
  <QUERY>  The query to search for

Options:
  -s, --sort <SORT>          Sort by (relevance, seeders, leechers, data, size) [default: relevance]
  -o, --order <ORDER>        The sort order (asc, desc) [default: desc]
  -c, --category <CATEGORY>  The category to search in (all, videos, software, music, games) [default: all]
  -p, --pages <PAGES>        The number of pages to show at one time [default: 1]
  -j, --json                 Output the results in JSON format
  -h, --help                 Print help
  -V, --version              Print version
```
