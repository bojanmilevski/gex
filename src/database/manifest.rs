use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Gecko {
	pub id: String,
	pub strict_min_version: Option<String>,
	pub strict_max_version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BrowserSpecificSettings {
	pub gecko: Gecko,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SuggestedKey {
	pub default: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExecuteBrowserAction {
	pub suggested_key: Option<SuggestedKey>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Commands {
	pub _execute_browser_action: Option<ExecuteBrowserAction>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContentScript {
	pub matches: Vec<String>,
	pub run_at: String,
	pub js: Vec<String>,
	pub css: Option<Vec<String>>,
	pub all_frames: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Background {
	pub scripts: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ThemeIcon {
	pub light: String,
	pub dark: String,
	pub size: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BrowserAction {
	pub browser_style: Option<bool>,
	pub default_icon: HashMap<String, String>,
	pub theme_icons: Option<Vec<ThemeIcon>>,
	pub default_popup: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Manifest {
	pub manifest_version: u8,
	pub name: String,
	pub version: String,
	pub author: String,
	pub description: String,
	pub default_locale: String,
	pub icons: HashMap<String, String>,
	pub browser_action: BrowserAction,
	pub permissions: Vec<String>,
	pub optional_permissions: Vec<String>,
	pub options_ui: Option<HashMap<String, String>>,
	pub background: Background,
	pub content_scripts: Vec<ContentScript>,
	pub commands: Commands,
	pub web_accessible_resources: Option<Vec<String>>,
	pub browser_specific_settings: BrowserSpecificSettings,
}
