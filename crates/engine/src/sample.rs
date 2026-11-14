//! The built-in sample screenplay: an original short scene written for DraftCraft
//! (dedicated to the public domain, CC0 1.0; see ATTRIBUTION.md).

use draftcraft_doc::Script;

/// The sample as Fountain.
pub const FOUNTAIN: &str = include_str!("sample.fountain");

/// The sample screenplay.
pub fn script() -> Script {
    // The one shot is marked `[[shot]]` (see the Fountain crate).
    draftcraft_fountain::parse(FOUNTAIN)
}
