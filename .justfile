[private]
default:
    @just --list --unsorted

[private]
edit:
    @$EDITOR .justfile

[linux]
asahi:
    cargo run --example asahi 
    xdg-open asahi_*.png > /dev/null 2>&1

[linux]
asahi2:
    cargo run --example asahi2
    xdg-open asahi2_*.png > /dev/null 2>&1
