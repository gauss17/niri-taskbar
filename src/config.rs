use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Deserializer};

/// The taskbar configuration.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    apps: HashMap<String, Vec<AppConfig>>,
    notifications: Option<Notifications>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Notifications {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    ignore_desktop_entry: bool,
    #[serde(default)]
    map_app_ids: HashMap<String, String>,
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
        self.notifications
            .as_ref()
            .map(|notifications| notifications.enabled)
            .unwrap_or(true)
    }

    /// Returns any mapping that might exist for this app ID.
    pub fn notifications_app_map(&self, app_id: &str) -> Option<&'_ str> {
        self.notifications
            .as_ref()
            .and_then(|notifications| notifications.map_app_ids.get(app_id))
            .map(|app_id| app_id.as_str())
    }

    /// Returns true if notification support should use the desktop entry as a
    /// fallback.
    pub fn notifications_should_use_desktop_entry(&self) -> bool {
        self.notifications
            .as_ref()
            .map(|notifications| !notifications.ignore_desktop_entry)
            .unwrap_or(true)
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
