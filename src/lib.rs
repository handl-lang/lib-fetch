//! `fetch` — Handl stdlib port contract.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl` — the `fetch` contract.
pub const FETCH_HNL: &str = include_str!("index.hnl");
