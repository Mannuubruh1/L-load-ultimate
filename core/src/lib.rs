//! Platform-independent config logic for L-Load.
//!
//! Nothing here touches the Switch, Skyline, or game memory, so it builds and
//! tests on any host (including Codespaces). The plugin crate will call into it.

use serde_json::{Map, Value};

/// Merge a freshly detected roster into an existing config.
///
/// - Existing entries (and their values) are preserved untouched.
/// - Newly detected characters are appended with `""` (unassigned).
/// - Characters that are no longer installed are kept, so uninstalling a mod
///   temporarily does not erase your links.
///
/// `existing` is the parsed contents of `l_load.json`, if any.
/// Returns the merged map, keeping existing key order first.
pub fn merge_roster(existing: Option<Map<String, Value>>, detected: &[String]) -> Map<String, Value> {
    let mut merged = existing.unwrap_or_default();
    for id in detected {
        if !merged.contains_key(id) {
            merged.insert(id.clone(), Value::String(String::new()));
        }
    }
    merged
}

/// Parse `l_load.json` text. Returns `None` if it is empty or not a JSON object,
/// so the caller can decide whether to regenerate or back up the bad file.
pub fn parse_config(text: &str) -> Option<Map<String, Value>> {
    match serde_json::from_str::<Value>(text) {
        Ok(Value::Object(map)) => Some(map),
        _ => None,
    }
}

/// Where pressing L from `from` leads, if a link is set.
/// An empty string means unassigned. A target that is not in the config at all
/// is treated as invalid and ignored, so a typo can never swap to nothing.
/// Costume-slot targets like `ui_chara_x:c120` are validated on the base id.
pub fn link_target<'a>(config: &'a Map<String, Value>, from: &str) -> Option<&'a str> {
    let target = config.get(from)?.as_str()?;
    let base = target.split(':').next().unwrap_or(target);
    if target.is_empty() || !config.contains_key(base) {
        return None;
    }
    Some(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn first_boot_writes_all_unassigned() {
        let merged = merge_roster(None, &ids(&["ui_chara_mario", "ui_chara_link"]));
        assert_eq!(merged.len(), 2);
        assert_eq!(merged["ui_chara_mario"], "");
    }

    #[test]
    fn merge_preserves_links_and_appends_new() {
        let existing = parse_config(r#"{"ui_chara_goku":"ui_chara_vegeta","ui_chara_vegeta":""}"#);
        let merged = merge_roster(existing, &ids(&["ui_chara_goku", "ui_chara_vegeta", "ui_chara_mario"]));
        assert_eq!(merged["ui_chara_goku"], "ui_chara_vegeta");
        assert_eq!(merged["ui_chara_mario"], "");
        let keys: Vec<_> = merged.keys().collect();
        assert_eq!(keys.last().unwrap().as_str(), "ui_chara_mario");
    }

    #[test]
    fn removed_characters_keep_their_links() {
        let existing = parse_config(r#"{"ui_chara_goku":"ui_chara_vegeta"}"#);
        let merged = merge_roster(existing, &ids(&["ui_chara_vegeta"]));
        assert_eq!(merged["ui_chara_goku"], "ui_chara_vegeta");
    }

    #[test]
    fn link_target_rules() {
        let cfg = parse_config(r#"{"a":"b","b":"a","c":"","d":"missing","e":"b:c120"}"#).unwrap();
        assert_eq!(link_target(&cfg, "a"), Some("b"));
        assert_eq!(link_target(&cfg, "c"), None); // unassigned
        assert_eq!(link_target(&cfg, "d"), None); // invalid target
        assert_eq!(link_target(&cfg, "e"), Some("b:c120")); // costume-slot syntax
        assert_eq!(link_target(&cfg, "zzz"), None); // not in config
    }

    #[test]
    fn bad_json_returns_none() {
        assert!(parse_config("").is_none());
        assert!(parse_config("[1,2]").is_none());
    }
}
