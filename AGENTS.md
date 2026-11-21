# Guide for contributors and coding agents

This is the GPL side of the project. The permissive side is
[osp-fo4](https://github.com/patodois/osp-fo4); its own guide is stricter than
this one and still applies to anything you send there.

## Licence

- Everything here is GPL-3.0-or-later. Start each source file with
  `// SPDX-License-Identifier: GPL-3.0-or-later`.
- A file ported from another project keeps that project's copyright notice
  and names the source revision in its header.

## One direction

- This repository depends on `osp-fo4`. Never copy code from here into
  `osp-fo4`.
- If you have read copyleft source for a format or system while working here,
  do not write the same format or system for `osp-fo4`. Send a measurement or
  a failing test there instead.

## Sources you may use

Code under a licence compatible with GPL-3.0-or-later, whose own origin is
clean: GPL-3.0, GPL-2.0-or-later, LGPL, MPL-2.0 (when the file is not marked
"Incompatible With Secondary Licenses"), Apache-2.0, MIT, BSD, Zlib.

Before using any of it:

1. Read the licence file of the exact revision you take, and the header of
   each file you take. A repository label is not enough.
2. Check where that code came from. See the next section.
3. Add the row to [THIRD_PARTY.md](THIRD_PARTY.md) in the same change.

## Sources that are never used, under any licence

- Game executables, and anything produced by decompiling or disassembling
  them. A project that publishes decompiled or matching-decompilation code
  under an open licence is still off limits, and so is code ported from one.
- Leaked or proprietary source, and code under an SDK or editor licence
  (for example the Havok SDK or the Creation Kit).
- Code with no licence, GPL-2.0-only code, and licences that add restrictions
  the GPL does not allow.

If a candidate source reads decompiled code, vendors a file of unclear
origin, or reproduces data tables taken from another tool, stop and open an
issue describing what you found. Do not use it until the origin is settled.

## What must never be committed

Game assets, images containing game textures, photographs of the game,
executable dumps, save files, local machine paths and secrets. Local output
goes under `build/` or `reports/`, which Git ignores.

## Checks

```sh
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

## Use of AI tools

Say so in the pull request when a change was written with an AI tool. The
rules above apply to the tool as they do to you: do not paste in, or ask it
to recall, any source listed as never used.
