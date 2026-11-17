// Integration test for SigmaOS-drag-drop
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use drag_drop as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
