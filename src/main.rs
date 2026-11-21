// SPDX-License-Identifier: GPL-3.0-or-later

//! Playable layer over the `osp-fo4` readers and renderer.
//!
//! Nothing is playable yet. This entry point only proves that the GPL side
//! builds against the permissive side at a pinned revision.

fn main() {
    println!(
        "osp-fo4-play {} (GPL-3.0-or-later), built on {}",
        env!("CARGO_PKG_VERSION"),
        core_name()
    );
}

/// Name of the permissive crate this binary links, taken from its type path so
/// the dependency is really used.
fn core_name() -> &'static str {
    let path = std::any::type_name::<fo4_core::Error>();
    path.split("::").next().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn links_the_permissive_core() {
        assert_eq!(super::core_name(), "fo4_core");
    }
}
