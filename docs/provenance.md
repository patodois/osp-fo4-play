# Provenance audit, phase 0

Licence and origin of third-party projects we are considering, read before any of
their code is ported or linked. Nothing here has been used yet. A row in
[THIRD_PARTY.md](../THIRD_PARTY.md) is still required for each actual use.

Method: LICENSE, README, CONTRIBUTING, manifests, file headers, directory
listings and commit metadata of the revision named, read through the GitHub API
and raw.githubusercontent.com on 2026-11-21. Implementation logic was not
studied and no code was copied. "Only" means the text names GPL version 3
without "or any later version"; combining such code with this repository makes
the combined work GPL-3.0 (only).

## Verdicts

| # | Project | Revision | Licence found | Verdict |
|---|---|---|---|---|
| 1 | Kalmarv/havok-formats | `fa26978` | GPL-3.0-or-later | UNDECIDED (black-box comparator until settled) |
| 2 | SquireWyrm/nifly-rs | `ba898a4` | GPL-3.0-or-later (own code) | USABLE IN GPL REPO |
| 2 | ousnius/nifly | `68bb808` (pin of nifly-rs) | GPL-3.0 only | USABLE IN GPL REPO |
| 3 | cobaltmods/fallout34havok | `2747b3a` | GPL-3.0-or-later (glue), GPL-3.0 (vendored) | USABLE AS BLACK-BOX COMPARATOR ONLY |
| 4 | Ortham/esplugin | `f964f38` | GPL-3.0, mixed only / or-later | USABLE IN GPL REPO |
| 4 | Ortham/libloadorder | `c9ea87f` | GPL-3.0, mixed only / or-later | USABLE IN GPL REPO |
| 4 | loot/libloot | `34bd01d` | GPL-3.0-or-later | USABLE IN GPL REPO |
| 5 | bmanzzs/fo4-lightbox | `6b52fed` | GPL-3.0 | DO NOT USE |
| 6 | dimforge/rapier | `475caa3` | Apache-2.0 | USABLE IN GPL REPO |
| 7 | openfo4/psc-tools | `ec8ebcb` | Apache-2.0 | USABLE IN GPL REPO |
| 7 | skyrim-multiplayer/skymp, `papyrus-vm/` only | `2849e67` | MIT | USABLE IN GPL REPO, except `CIString.h` |
| 7 | NOT-REAL-GAMES/OpenF4, `third_party/papyrus-vm/` only | `8a48ac8` | MIT (upstream part), none stated for OpenF4's changes | UNDECIDED |
| 8 | ruffle-rs/ruffle | `0ddd12b` | MIT OR Apache-2.0 | USABLE IN GPL REPO |
| 9 | Dexesttp/hkxpack | `659f1f0` | MIT | DO NOT USE |
| 10 | vgmstream/vgmstream | `24ffd6c` | ISC, plus bundled libraries | USABLE AS BLACK-BOX COMPARATOR ONLY |
| 11 | mdfairch/FallrimTools | `61bb57d` | Apache-2.0, some files other | USABLE IN GPL REPO, Apache-headed files only |
| 12 | LootJunkieAzza/NVForever | `9e6d604` | GPL-3.0-or-later | USABLE IN GPL REPO (docs and method only) |

"USABLE AS BLACK-BOX COMPARATOR ONLY" means: run its published program on our
own inputs and compare outputs. Do not read its source, link it or port it.

---

## 1. Kalmarv/havok-formats

- Revision: `fa2697877d204b7680907bedcb7854db03389b65` (main, 2026-11-06). Rust.
- Licence: `LICENSE` is the GPL-3.0 text; `Cargo.toml` says
  `license = "GPL-3.0-or-later"`; README says "Licensed under GPL-3.0-or-later".
  No per-file SPDX headers. One licence for the whole tree.
- Dependencies by the same author: `primase-foundations`, `primase-testdata`,
  both GPL-3.0-or-later on crates.io. Their own provenance was not audited.
- Game coverage: written against Sekiro, Elden Ring and a PS3 title. It reads
  `hk_2014.1.0-r1` packfiles, the version Fallout 4 uses, but Fallout 4 is not
  among the tested games.
- Decompilation or SDK: README says it "contains no Havok SDK code". However,
  `src/enums.rs` and `src/types.rs` take enum member names and numbering from
  the Havok 6.6 SDK headers ("the 6.6 SDK's numbering"; "checked against the
  Havok 6.6 headers"). Other enum names come from HKLib (MIT), whose type
  registry files (`HavokTypeRegistry20180100.xml` and others) state no origin
  and include Havok SDK test classes. `src/spline_encode.rs` reproduces the
  arithmetic of `CompressAnim.exe` (a Havok 2010.2 tool) by comparing its
  outputs.
  - "the 6.6 SDK's numbering" —
    https://github.com/Kalmarv/havok-formats/blob/fa2697877d204b7680907bedcb7854db03389b65/src/enums.rs
  - "checked against the Havok 6.6 headers" —
    https://github.com/Kalmarv/havok-formats/blob/fa2697877d204b7680907bedcb7854db03389b65/CHANGELOG.md
- Test fixtures: none committed; real-data tests read the user's own game files
  through a config (`tests/inputs/README.md`).
- AI policy: none stated.
- Verdict: **UNDECIDED.** AGENTS.md treats code under an SDK licence and data
  tables taken from another tool as reasons to stop. Missing: whether enum
  names and values copied from Havok SDK headers are acceptable, and where
  HKLib's type registries come from. Until then, comparator only. If settled,
  the parts that do not depend on those tables could be reconsidered.

## 2. SquireWyrm/nifly-rs and ousnius/nifly

### nifly-rs

- Revision: `ba898a4b9b8118b7a3f2e1a5db81a83b3c7b5612` (master, 2026-11-21). Rust,
  plus C++ shims.
- Licence: GPL-3.0 text in `LICENSE`; both crates declare
  `GPL-3.0-or-later`. README: "nifly, which is built into the crate, is
  licensed under the GPL v3."
- Vendored: ousnius/nifly as a git submodule at `nifly-sys/vendor/nifly`,
  pinned to `68bb80893a65b1f17393bb21c9ad803add2594d8`.
- Test fixtures: `assets/*.nif`, described as made for the project, some derived
  from nifly's own test files. `CLAUDE.md` says meshes extracted from a game are
  kept in a gitignored folder and never committed.
- AI policy: README states "Much of this crate's code, tests and documentation
  was written with Claude". No restriction on contributions.
- Provenance: a binding over nifly; no sign of decompilation.

### nifly (pinned revision)

- Revision: `68bb80893a65b1f17393bb21c9ad803add2594d8` (2026-11-19). C++.
  Upstream head on the audit date is `e6d852d`, not inspected.
- Licence: `LICENSE` is the GPL-3.0 text. File headers say "See the included
  GPLv3 LICENSE file", with no "or later". Treat as **GPL-3.0 only**.
- Vendored: `external/half.hpp` (MIT), `external/Miniball.hpp`
  (GPL-3.0-or-later). Catch2 for tests, not vendored.
- Provenance: "Created with a clean-room design." —
  https://github.com/ousnius/nifly/blob/68bb80893a65b1f17393bb21c9ad803add2594d8/README.md
- Test fixtures: 40 files in `tests/input/`, including Fallout 4 and Fallout 76
  NIFs up to 168 KB. Their origin is not stated. Do not copy them.
- AI policy: none stated.
- Verdict: **USABLE IN GPL REPO** for linking. The build is then GPL-3.0 only.
  Keep the nifly test files out of this repository.

## 3. cobaltmods/fallout34havok

- Revision: `2747b3a9c57d79febbc25c30550a0effca856e77` (main, 2026-11-05). Python.
- Licence: GPL-3.0 text in `LICENSE`. Glue files carry
  "GPL-3.0-or-later (see LICENSE)". The README credits table lists the vendored
  files as GPL-3.0.
- History: a single root commit with no parent, authored by the account's
  noreply address. 16 files.
- Relation to cobaltmods/fo4test (README, licence, file names and history only):
  - fo4test is a fork of `jarari/fo4test`, an F4SE rendering plugin built on
    CommonLibF4. Its 109 commits are by three other authors; none by this
    account.
  - No blob hash is shared between the two trees. The only shared file names
    are `.gitignore`, `LICENSE` and `README.md`, and their contents differ.
    fo4test has no Havok, HKX or collision files.
  - No file in fallout34havok mentions fo4test or its authors.
  - Conclusion: no shared code, tables or history was found.
- Vendored chain:
  - `vendor/pyn/bhk_autopack.py`, `bhk_autounpack.py`: identical to
    BadDogSkyrim/PyNifly tag `V29.1.0` (`650375a`) apart from line endings.
  - `vendor/hkx/anim_fo4.py`: PyNifly `V29.1.0` plus a three-line header, the
    marked root-motion change, and one unmarked comment change.
  - PyNifly: GPL-3.0 text, no "or later" statement; several commits touching
    the packer are co-authored by Claude.
  - `anim_fo4.py` says it is "Based on spline decompression from Dagobaking's
    skyrim-fo4-animation-conversion (itself based on PredatorCZ/HavokLib,
    GPL-3.0)". The Dagobaking repository could not be found on GitHub, so its
    licence is unverified.
  - PredatorCZ/HavokLib: GPL v3 by README, no "or later" seen. Class layouts
    for Havok 5.0 to 2017 are generated per version under `classgen/`; their
    source is not stated. Test resources are a compressed archive of compiled
    files of unstated origin.
- Game data inside the code: the packer embeds byte blocks copied from vanilla
  Fallout 4 files.
  - Two comments in the file below name the vanilla meshes the blocks were
    taken from:
    https://github.com/cobaltmods/fallout34havok/blob/2747b3a9c57d79febbc25c30550a0effca856e77/f34havok/vendor/pyn/bhk_autopack.py
  - Class layouts are credited "hk2014 SDK, via Grimrukh/soulstruct-havok";
    that project was not audited.
- AI policy: none stated in this repository.
- Verdict: **USABLE AS BLACK-BOX COMPARATOR ONLY.** Do not port: it carries
  bytes taken from game files and an upstream link whose licence cannot be
  checked.

## 4. Ortham/esplugin, Ortham/libloadorder, loot/libloot

### esplugin

- Revision: `f964f3895747362ea6ce4cbbea56f2a746c39319` (master, 2026-10-26). Rust.
- Licence: `LICENSE` is the GPL-3.0 text. `Cargo.toml` says
  `license = "GPL-3.0"` (the old SPDX id, read as only). 8 of 16 source files
  have a header with "either version 3 of the License, or (at your option) any
  later version"; the `ffi/` crate and the benchmark have no header. Treat the
  crate as GPL-3.0 only.
- Tests download `Ortham/testing-plugins` (MIT): plugins made by hand or with
  xEdit, with no game records ("hand-made in a hex editor", "header-only").
- Provenance: no sign of decompilation.
- AI policy: none stated.
- Verdict: **USABLE IN GPL REPO** (combined work GPL-3.0 only).

### libloadorder

- Revision: `c9ea87f9ce22095a4bc593c89a7c4ea2eb5e72ad` (master, 2026-10-26). Rust.
- Licence: same pattern. `license = "GPL-3.0"`; 21 of 25 source files carry
  the "or later" header; `src/load_order/openmw.rs`, `src/openmw_config.rs`,
  the benchmark and the FFI test do not.
- Tests: `Ortham/testing-plugins`, as above.
- Verdict: **USABLE IN GPL REPO** (combined work GPL-3.0 only).

### libloot

- Revision: `34bd01dbd55eb2ba4c0d3976a95a4ab01c1cf2f1` (master, 2026-10-29). Rust,
  with C++, Python and Node bindings.
- Licence: workspace `license = "GPL-3.0-or-later"`; `LICENSE` is the GPL-3.0
  text; documentation under GFDL-1.3-or-later.
- Depends on esplugin, libloadorder (above) and `loot-condition-interpreter`
  (MIT).
- The notice asks those making derivative works to talk to the team first, and
  asks people not to distribute unofficial masterlists. These are requests, not
  licence terms.
- Verdict: **USABLE IN GPL REPO.** Because of its dependencies, a build that
  links it is GPL-3.0 only.

## 5. FO4 Lightbox (bmanzzs/fo4-lightbox)

- Found through the FO4 report. Revision: `6b52fed7d9184f1df3bd65fb5a78322ea98f4787`
  (main, 2026-10-26). JavaScript, GLSL, Python, C#.
- Licence: GPL-3.0 text; README says "GPL-3.0", no "or later".
- Vendored: PyNifly `anim_fo4.py` unchanged (same chain as item 3), Rapier
  JavaScript build (Apache-2.0).
- Decompilation: the project's own development rule allows decompiling the
  game's shaders, keeps the output in a private folder outside the repository,
  and writes the GLSL from notes based on it. Its Havok cloth reader takes class
  layouts from the game executable.
  - "Decompiling `Fallout 4 - Shaders.ba2` to learn exactly what the game does
    is fine" —
    https://github.com/bmanzzs/fo4-lightbox/blob/6b52fed7d9184f1df3bd65fb5a78322ea98f4787/docs/DEVELOPMENT.md
  - "Class layouts come from the hkClassMember tables inside Fallout4.exe" —
    https://github.com/bmanzzs/fo4-lightbox/blob/6b52fed7d9184f1df3bd65fb5a78322ea98f4787/server/havok.py
- Test fixtures: none; `scenes/LightboxSimple.esp` is the project's own plugin.
- AI policy: README says it was "Written with Claude".
- Verdict: **DO NOT USE**, including its docs (`docs/engine-notes.md` is
  derived from the disassembled shaders). AGENTS.md excludes anything produced
  by decompiling or disassembling game code.

## 6. dimforge/rapier

- Revision: `475caa3ff4646fd9a796b67d14116256f4962bc7` (master, 2026-10-10). Rust.
- Licence: `LICENSE` is Apache-2.0; root and crate manifests say
  `license = "Apache-2.0"`. Dependencies `parry3d` and `nalgebra` are
  Apache-2.0 on crates.io.
- Per-subtree: the C testbed under `bindings/c/testbed/` ships third-party
  material under LGPL-2.1, MIT, CC0, OFL and others (`licenses/SOURCES.md`). It
  is not part of the Rust crates.
- Provenance: a general physics engine; nothing ties it to game or Havok code.
- AI policy: none found in `CONTRIBUTING.md`.
- Verdict: **USABLE IN GPL REPO.** Apache-2.0 is compatible with GPL-3.0.

## 7. Papyrus: psc-tools, Skymp VM, OpenF4 adapter

### openfo4/psc-tools

- Revision: `ec8ebcb59b9f861242a022c370554f5a804a6847` (master, 2026-11-17). Rust.
- Licence: Apache-2.0 in the root `LICENSE` only. No `license` field in any
  crate manifest and no file headers. Single author in the history.
- Contents: `pex` reader, `pex-as` assembler, `pscc` compiler, `pex-runner`.
  Test `.psc` scripts are small project-written files.
- Provenance: no statement of how the PEX format or the compiler semantics were
  learned; no sign of decompilation. The same organisation hosts `openfo4/openfo4`
  (Apache-2.0), not audited here.
- AI policy: "We do not accept AI generated content." —
  https://github.com/openfo4/psc-tools/blob/ec8ebcb59b9f861242a022c370554f5a804a6847/README.md
  This rules out sending them AI-written contributions; it does not restrict use.
- Verdict: **USABLE IN GPL REPO.** Record the absent origin statement in
  THIRD_PARTY.md when used.

### skyrim-multiplayer/skymp, `papyrus-vm/` only

- Revision: `2849e676fe008925384b26f34476e382a30bd877` (main, 2026-10-05). C++.
- Licence: `papyrus-vm/LICENSE` is MIT. It was AGPL-3.0 until commit
  `1bbce06` (2024-01-26, "update papyrus-vm LICENSE"). The project's CLA assigns
  copyright to the team, which allows that relicence. Earlier revisions are
  AGPL-3.0: take nothing older than `1bbce06`.
- Other subtrees differ: `TERMS.md` says the project is "primarily distributed
  under the GPLv3 and AGPLv3"; `skymp5-server` is AGPL; `THIRD_PARTY_LICENSES`
  lists components marked "All Rights Reserved". None of these are needed by
  the VM.
- VM build dependencies: `viet` (MIT), `MakeID.h` (public domain), spdlog.
- Per-file issue: `include/papyrus-vm/CIString.h` cites two Stack Overflow
  answers. Stack Overflow content is CC BY-SA, and versions before 4.0 are not
  compatible with the GPL. Rewrite that file instead of porting it.
- Provenance: no statement of method; no reference to game executables,
  CommonLib or disassembly in the VM sources.
- AI policy: none; `CLAUDE.md` holds build tips only.
- Verdict: **USABLE IN GPL REPO** at `1bbce06` or later, without `CIString.h`.

### NOT-REAL-GAMES/OpenF4, `third_party/papyrus-vm/` only

- Revision: `8a48ac89aedd8cae4e6d0490c52cd279a8eeaff6` (main, 2026-10-12). C++.
- Licence: no root licence file. The subtree's `LICENSE` is Skymp's MIT text
  with Skymp's copyright only. OpenF4's own changes (for example `Fo4Reader.cpp`)
  carry no header; the README calls the result a "standalone, MIT-licensed
  derivative of SkyMP's Papyrus VM".
- `UPSTREAM.md` pins Skymp at the same revision audited above.
- Provenance: "This project does not and will never rely on decompilation
  efforts" —
  https://github.com/NOT-REAL-GAMES/OpenF4/blob/8a48ac89aedd8cae4e6d0490c52cd279a8eeaff6/README.md
- Test fixtures: `tests/fixtures/papyrus/compiled.hpp` embeds PEX bytes made
  with the Creation Kit's Papyrus compiler against the game's base script
  sources. Exclude it.
- AI policy: none stated.
- Verdict: **UNDECIDED.** Missing: a licence grant with an author's copyright
  covering OpenF4's own changes (a README sentence is thin). Prefer the Skymp
  upstream and write the Fallout 4 reader ourselves.

## 8. ruffle-rs/ruffle

- Revision: `0ddd12b6adf211f556a375e96fca70095de5835b` (master, 2026-11-20). Rust.
- Licence: `LICENSE.md` and workspace manifest: `MIT OR Apache-2.0`.
- Per-subtree: `tests/tests/swfs/from_avmplus/` holds tests adapted from Adobe's
  avmplus under MPL-2.0. Test-only; do not copy.
- Provenance: "It is strictly forbidden to decompile Flash Player, Flash
  Professional, Adobe Animate, or any other software that does not explicitly
  permit doing so." —
  https://github.com/ruffle-rs/ruffle/blob/0ddd12b6adf211f556a375e96fca70095de5835b/CONTRIBUTING.md
- AI policy: LLM-written code is allowed if labelled (based on LLVM's policy).
- Verdict: **USABLE IN GPL REPO.** Whether it can play the game's Scaleform
  menus is a separate, technical question.

## 9. Dexesttp/hkxpack

- Revision: `659f1f03185aac7c48272809b9d05270b58b73fa` (main, 2020-11-11). Java.
- Licence: MIT in `LICENSE`.
- Provenance: the class definitions the tool needs are taken from the game and
  are not in the repository; the method is not published. The tool does not run
  without them.
  - "the 'classXML' files are obtained from the Fallout 4 game" —
    https://github.com/Dexesttp/hkxpack/blob/659f1f03185aac7c48272809b9d05270b58b73fa/README.md
- Test fixtures: `core/src/test/resources/test-base.hkx`, origin not stated.
  `doc/hkx findings/` analyses specific game files.
- AI policy: none.
- Verdict: **DO NOT USE.** It depends on class data extracted from the game by
  an unpublished method.

## 10. vgmstream/vgmstream

- Revision: `24ffd6ce3e1f34fdebf9f5316d18ad2e162838ce` (master, 2026-11-10). C.
- Licence: `COPYING` is ISC, with several "Portions Copyright" notices. Bundled
  prebuilt libraries in `ext_libs/` (FFmpeg, mpg123, Vorbis, Speex, CELT) have
  their own licences in `ext_libs/licenses/`.
- Decompilation: 44 of 161 decoder files under `src/coding/` say they were
  decompiled or reverse engineered from game executables or DLLs. The Wwise
  Vorbis codebooks are "Extracted from Wwise SDK by ww2ogg".
  - "decompiled from the exe" —
    https://github.com/vgmstream/vgmstream/blob/24ffd6ce3e1f34fdebf9f5316d18ad2e162838ce/src/coding/derf_decoder.c
- AI policy: not checked.
- Verdict: **USABLE AS BLACK-BOX COMPARATOR ONLY.** Do not link or port; the
  source mixes decompiled code into the library. For Fallout 4 audio, use
  separately licensed codecs.

## 11. mdfairch/FallrimTools

- Revision: `61bb57d895f023617cb55045a4907849fd1ff566` (main, 2023-10-01). Java.
- Licence: `LICENSE` and most file headers are Apache-2.0.
- Files with other terms:
  - `net/infotrek/util/prefs/*`: no licence;
  - `resaver/gui/WrapLayout.java`: taken from a blog, no licence named;
  - `resaver/gui/AutoCompletion.java`: public domain;
  - Stack Overflow snippets in `SupportDialog.java`, `FragmentBase.java`,
    `GeneralElement.java`.
  Runtime dependency `juniversalchardet` is listed under MPL-1.1. An icon is
  CC-BY-SA-2.0. A prebuilt `pack/ReSaver.exe` launcher and `target/` output are
  committed.
- Provenance: no statement of how the save format was learned. "Disassembly" in
  the sources refers to the Papyrus scripts stored in saves, not to the game
  executable. No sign of executable decompilation.
- Test fixtures: no save or plugin files in the tree.
- AI policy: none.
- Verdict: **USABLE IN GPL REPO** for files with the Apache-2.0 header only.
  Leave out the files listed above. The GitHub repository was last pushed in
  2023; check whether it is the current upstream before taking anything.

## 12. LootJunkieAzza/NVForever (docs and method only)

- Revision: `9e6d6047dd1ef5420c2252c7ff215e44d3dc56c2` (main, 2026-11-21). Rust.
- Licence: GPL-3.0-or-later (`LEGAL.md`, `Cargo.toml`). Contributions under the
  Developer Certificate of Origin.
- Clean-room rules (`LEGAL.md`): sources are public documentation, the user's
  own data files, and the running game watched through xNVSE's public plugin
  interface. No decompiled or disassembled game or editor code, no leaked
  sources or symbols. Of the executable, only unencrypted data (version, imports,
  plain strings) was read. Each claim in `docs/spec-notes/` is tagged
  documented, observed or hypothesis.
  - "No decompiled or disassembled game or editor code." —
    https://github.com/LootJunkieAzza/NVForever/blob/9e6d6047dd1ef5420c2252c7ff215e44d3dc56c2/LEGAL.md
- Data taken from xEdit: record definitions (MPL-2.0) are read as documentation
  and re-expressed in `schemas/`. One file, `schemas/script/functions.json`, is
  extracted by script from `wbDefinitionsFNV.pas`: script function names,
  condition indices and parameter types.
- Disclosed exception: before 2026-10-30 the project used nv-rs once to
  cross-check NIF animation layouts. It states that nothing was taken and the
  check was later replaced by data alone. nv-rs is on our never-open list, so
  do not reuse layout notes from that period without checking their source tags.
- AI policy: much of the project is written with AI assistants and AI-assisted
  contributions are accepted.
- Verdict: **USABLE IN GPL REPO** as a model for our own docs and method. Its
  code was not audited and is not covered by this verdict.

## Not verified

- Dagobaking's skyrim-fo4-animation-conversion: no public repository found.
- Origin of HavokLib's per-version class layouts and test archive; of HKLib's
  type registries; of Grimrukh/soulstruct-havok's layouts.
- Origin of nifly's and hkxpack's test files.
- Provenance of `primase-foundations` and `primase-testdata`.
- How psc-tools, the Skymp VM and FallrimTools learned the formats they read:
  no statement either way.
- vgmstream's contribution policy on AI code.
