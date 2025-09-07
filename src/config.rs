use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Deserializer};

/// The taskbar configuration.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    apps: HashMap<String, Vec<AppConfig>>,
    #[serde(default)]
    notifications: Notifications,
    #[serde(default)]
    show_all_outputs: bool,
    #[serde(default)]
    orientation: Orientation,
    #[serde(default)]
    workspace_format: String,
    #[serde(default)]
    workspace_format_focused: String,
}

#[derive(Debug, Deserialize)]
pub struct Notifications {
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default)]
    map_app_ids: HashMap<String, String>,
    #[serde(default = "default_true")]
    use_desktop_entry: bool,
    #[serde(default)]
    use_fuzzy_matching: bool,
}

impl Default for Notifications {
    fn default() -> Self {
        Self {
            enabled: true,
            map_app_ids: Default::default(),
            use_desktop_entry: true,
            use_fuzzy_matching: Default::default(),
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Orientation {
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "horizontal")]
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}

impl Config {
    /// Returns all possible CSS classes that a particular application might have set.
    pub fn app_classes(&self, app_id: &str) -> Vec<&str> {
        self.apps
            .get(app_id)
            .map(|configs| {
                configs
                    .iter()
                    .map(|config| config.class.as_str())
                    .collect_vec()
            })
            .unwrap_or_default()
    }

    /// Returns the actual CSS classes that should be set for the given application and title.
    pub fn app_matches<'a>(
        &'a self,
        app_id: &str,
        title: &'a str,
    ) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        match self.apps.get(app_id) {
            Some(configs) => Box::new(
                configs
                    .iter()
                    .filter(|config| config.re.is_match(title))
                    .map(|config| config.class.as_str()),
            ),
            None => Box::new(std::iter::empty()),
        }
    }

    /// Returns true if notification support is enabled.
    pub fn notifications_enabled(&self) -> bool {
        self.notifications.enabled
    }

    /// Returns any mapping that might exist for this app ID.
    pub fn notifications_app_map(&self, app_id: &str) -> Option<&'_ str> {
        self.notifications
            .map_app_ids
            .get(app_id)
            .map(String::as_str)
    }

    /// Returns true if notification support should use the desktop entry as a
    /// fallback.
    pub fn notifications_use_desktop_entry(&self) -> bool {
        self.notifications.use_desktop_entry
    }

    pub fn notifications_use_fuzzy_matching(&self) -> bool {
        self.notifications.use_fuzzy_matching
    }

    pub fn show_all_outputs(&self) -> bool {
        self.show_all_outputs
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn workspace_format(&self) -> &str {
        &self.workspace_format
    }

    pub fn workspace_format_focused(&self) -> &str {
        &self.workspace_format_focused
    }
}

#[derive(Deserialize, Debug)]
struct AppConfig {
    #[serde(rename = "match", deserialize_with = "deserialise_regex")]
    re: Regex,
    class: String,
}

fn deserialise_regex<'de, D>(de: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    Regex::new(&String::deserialize(de)?).map_err(serde::de::Error::custom)
}
