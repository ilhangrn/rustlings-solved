# [Rustlings](https://rustlings.rust-lang.org) 🦀

Small exercises to get you used to reading and writing [Rust](https://www.rust-lang.org) code - _Recommended in parallel to reading [the official Rust book](https://doc.rust-lang.org/book) 📚️_

Visit the **website** for a demo, info about setup and more:

## ➡️ [rustlings.rust-lang.org](https://rustlings.rust-lang.org) ⬅️

## Nix Development

This repository uses flakes for the dev environment.

- Enter the shell with: nix develop
- Pin project Nix outputs against garbage collection: nix run .#lock
- Legacy shell.nix has been removed

The lock command creates project-local GC roots in .nix-gc-roots/. The dev shell
root is always updated. The package root is best-effort and may be skipped while
exercises are incomplete.

If you do not have flakes enabled yet, enable Nix flakes in your local Nix config first.

```bash
mkdir -p ~/.config/nix && nano ~/.config/nix/nix.conf
```

add `experimental-features = nix-command flakes` to nix.conf file.