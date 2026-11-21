# osp-fo4-play

The playable layer of the open Fallout 4 reimplementation, licensed
**GPL-3.0-or-later**. It needs a legally owned copy of the game: no game data
is here, and none may be added.

Nothing is playable yet. This repository exists so that work which builds on
GPL code has a home that is kept apart from the permissive side.

## Two repositories, one direction

| Repository | Licence | What lives there |
|---|---|---|
| [osp-fo4](https://github.com/patodois/osp-fo4) | MIT OR Apache-2.0 | Readers, scene building, reference renderer and viewer, written from measurement and public format documentation only. |
| osp-fo4-play (this one) | GPL-3.0-or-later | Game logic and systems that may link or port GPL-compatible code of clean provenance. |

This repository depends on `osp-fo4` at a pinned revision. Code moves from
`osp-fo4` to here and never the other way, so the permissive side stays usable
by anyone under its own terms.

## Build

```sh
cargo test
cargo run
```

## Contributing

Read [AGENTS.md](AGENTS.md) first. It says which sources may be used, which
never may, and how each use is recorded in [THIRD_PARTY.md](THIRD_PARTY.md).
