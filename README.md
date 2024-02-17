# swc-minify
Simple binary wrapper around SWC to minify JS files. No custom config supported yet - just specify input file and it's done.

## Motivation
I created this wrapper mostly because `swc minify` from `swc_cli` crate says "Minify command is not yet implemented".

## Install
```
cargo install swc-minify
```
## Usage
```
swc-minify main.js > main.min.js
```
## Notes
Compress and mangle options are used.
