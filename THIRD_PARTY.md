# Third-party code

Every file that links, ports or adapts code from another project is listed
here before it is merged. One row per source.

| Source repository | Revision | Licence (as read in that revision) | What was used | Files here |
|---|---|---|---|---|
| [osp-fo4](https://github.com/patodois/osp-fo4) | `6c48f91e4bf6bcedbd0a9adc7f522fcc9c7be496` | MIT OR Apache-2.0 | Linked crate `fo4-core` | whole tree |

No code from any other project has been ported or adapted.

## Crates linked through `fo4-core`

The exact versions are in `Cargo.lock`. Licences as declared in each crate's
manifest; none narrows this crate's licence.

| Crate | Version | Licence |
|---|---|---|
| adler2 | 2.0.1 | 0BSD OR MIT OR Apache-2.0 |
| block-buffer | 0.10.4 | MIT OR Apache-2.0 |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 |
| cpufeatures | 0.2.17 | MIT OR Apache-2.0 |
| crc32fast | 1.5.5 | MIT OR Apache-2.0 |
| crypto-common | 0.1.7 | MIT OR Apache-2.0 |
| digest | 0.10.7 | MIT OR Apache-2.0 |
| flate2 | 1.1.11 | MIT OR Apache-2.0 |
| generic-array | 0.14.7 | MIT |
| itoa | 1.0.19 | MIT OR Apache-2.0 |
| libc | 0.2.191 | MIT OR Apache-2.0 |
| memchr | 2.8.3 | Unlicense OR MIT |
| miniz_oxide | 0.9.1 | MIT OR Zlib OR Apache-2.0 |
| serde, serde_core, serde_derive | 1.0.229 | MIT OR Apache-2.0 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 |
| sha2 | 0.10.9 | MIT OR Apache-2.0 |
| simd-adler32 | 0.3.10 | MIT |
| typenum | 1.20.1 | MIT OR Apache-2.0 |
| version_check | 0.9.5 | MIT OR Apache-2.0 |
| zmij | 1.0.23 | MIT |
| proc-macro2, quote, syn, unicode-ident (build time, for `serde_derive`) | 1.0.107, 1.0.47, 3.0.10, 1.0.27 | MIT OR Apache-2.0; unicode-ident also Unicode-3.0 |
