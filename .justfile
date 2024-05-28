[private]
default:
    @just --list --unsorted

[private]
edit:
    @$EDITOR .justfile

asahi:
    cargo run --example asahi 
    xdg-open asahi.png > /dev/null 2>&1
