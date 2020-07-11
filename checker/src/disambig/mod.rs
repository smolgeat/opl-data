//! Auto-disambiguation for lifters with the same username.

use crate::{AllMeetData, LifterMap};

/// Performs auto-disambiguation.
///
/// Consumes the original LifterMap, producing an updated LifterMap.
/// The AllMeetData entries are modified to match new disambiguations.
///
pub fn infer(liftermap: LifterMap, meetdata: &mut AllMeetData) -> LifterMap {
    for username in liftermap.keys() {
        // Inference only checks un-disambiguated usernames: "janedoe", not "janedoe1".
        if username.ends_with(|c: char| c.is_digit(10)) {
            continue;
        }

        infer_for(username, &liftermap, meetdata);
    }

    // Generate a new LifterMap from the changed AllMeetData.
    meetdata.create_liftermap()
}


/// Performs auto-disambiguation for a single base username.
pub fn infer_for(username: &str, liftermap: &LifterMap, meetdata: &mut AllMeetData) {
}
