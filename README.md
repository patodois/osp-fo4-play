# osp-fo4-play

<!-- images: title, licence-direction diagram (osp-fo4, MIT/Apache -> osp-fo4-play, GPL; never back) -->

The playable layer of the open Fallout 4 reimplementation, licensed
**GPL-3.0-or-later**. It builds on the permissive readers and renderer of
[osp-fo4](https://github.com/patodois/osp-fo4) and is where code that links or
ports GPL-compatible projects will live, kept apart from the permissive side.

## What it does today

- One binary that links `fo4-core` from osp-fo4 at a pinned revision and prints
  one line naming itself and that crate. It proves the GPL side builds against
  the permissive side; nothing is playable yet.
- A licence and origin audit of projects we may use: [docs/provenance.md](docs/provenance.md).

## What it does not do

- It holds no game data and needs a legally owned copy of the game; no game
  files, images of the game or save files may be added.
- No code moves from here into osp-fo4.
- No third-party code is used yet ([THIRD_PARTY.md](THIRD_PARTY.md)).

## Try it

The first build fetches the pinned osp-fo4 revision and crates.io packages, so
it needs network access. With Rust 1.88 or newer:

```sh
cargo test --locked
cargo run --locked
```

`cargo test` runs one test (`links_the_permissive_core`). `cargo run` prints
`osp-fo4-play 0.0.0 (GPL-3.0-or-later), built on fo4_core` and exits.

## Two repositories, one direction

| Repository | Licence | What lives there |
|---|---|---|
| [osp-fo4](https://github.com/patodois/osp-fo4) | MIT OR Apache-2.0 | Readers, scene building, reference renderer and viewer, written from measurement and public format documentation only. |
| osp-fo4-play (this one) | GPL-3.0-or-later | Game logic and systems that may link or port GPL-compatible code of clean provenance. |

This repository depends on `osp-fo4` at a pinned revision. Code moves from
`osp-fo4` to here and never the other way, so the permissive side stays usable
by anyone under its own terms.

## Contributing

Read [AGENTS.md](AGENTS.md) first. It says which sources may be used, what
each licence does to this crate's licence, which sources never may be used, and
how each use is recorded in [THIRD_PARTY.md](THIRD_PARTY.md).
