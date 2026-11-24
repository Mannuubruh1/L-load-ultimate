//! L-Load plugin entry point.
//!
//! Phase 2 skeleton: loads, logs, and proves the shared `l_load_core` logic
//! links into the plugin. Real work (roster, config I/O, CSS hooks) comes next.

#[skyline::main(name = "l_load")]
pub fn main() {
    // Exercise the core crate so a bad link shows up at build time.
    let empty = l_load_core::merge_roster(None, &[]);
    println!("[l_load] loaded (config entries so far: {})", empty.len());
}
