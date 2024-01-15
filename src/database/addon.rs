use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Addons {
	#[serde(rename = "schemaVersion")]
	pub schema_version: u32,
	pub addons: Vec<Addon>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Addon {
	pub id: String,
	#[serde(rename = "syncGUID")]
	pub sync_guid: Option<String>,
	pub version: String,
	#[serde(rename = "type")]
	pub _type: String,
	pub loader: Option<String>,
	#[serde(rename = "updateURL")]
	pub update_url: Option<String>,
	#[serde(rename = "installOrigins")]
	pub install_origins: Option<String>,
	#[serde(rename = "manifestVersion")]
	pub manifest_version: u8,
	#[serde(rename = "optionsURL")]
	pub options_url: Option<String>,
	#[serde(rename = "optionsType")]
	pub options_type: Option<u32>,
	#[serde(rename = "optionsBrowserStyle")]
	pub options_browser_style: bool,
	#[serde(rename = "aboutURL")]
	pub about_url: Option<String>,
	#[serde(rename = "defaultLocale")]
	pub default_locale: Option<DefaultLocale>,
	pub visible: bool,
	pub active: bool,
	#[serde(rename = "userDisabled")]
	pub user_disabled: bool,
	#[serde(rename = "appDisabled")]
	pub app_disabled: bool,
	#[serde(rename = "embedderDisabled")]
	pub embedder_disabled: bool,
	#[serde(rename = "installDate")]
	pub install_date: Option<i64>,
	#[serde(rename = "updateDate")]
	pub update_date: Option<i64>,
	#[serde(rename = "applyBackgroundUpdates")]
	pub apply_background_updates: u32,
	pub path: Option<PathBuf>,
	pub skinnable: bool,
	#[serde(rename = "sourceURI")]
	pub source_uri: Option<String>,
	#[serde(rename = "releaseNotesURI")]
	pub release_notes_uri: Option<String>,
	#[serde(rename = "softDisabled")]
	pub soft_disabled: bool,
	#[serde(rename = "foreignInstall")]
	pub foreign_install: bool,
	#[serde(rename = "strictCompatibility")]
	pub strict_compatibility: bool,
	pub locales: Vec<Locale>,
	#[serde(rename = "targetApplications")]
	pub target_applications: Vec<TargetApplication>,
	#[serde(rename = "targetPlatforms")]
	pub target_platforms: Vec<String>,
	#[serde(rename = "signedState")]
	pub signed_state: Option<u8>,
	#[serde(rename = "signedDate")]
	pub signed_date: Option<u64>,
	pub seen: Option<bool>,
	pub dependencies: Vec<String>,
	pub incognito: Option<String>,
	#[serde(rename = "userPermissions")]
	pub user_permissions: Option<Permissions>,
	#[serde(rename = "optionalPermissions")]
	pub optional_permissions: Option<Permissions>,
	pub icons: HashMap<String, String>,
	#[serde(rename = "iconURL")]
	pub icon_url: Option<String>,
	#[serde(rename = "blocklistState")]
	pub blocklist_state: Option<u32>,
	#[serde(rename = "blocklistURL")]
	pub blocklist_url: Option<String>,
	#[serde(rename = "startupData")]
	pub startup_data: Option<HashMap<String, String>>,
	pub hidden: Option<bool>,
	#[serde(rename = "installTelemetryInfo")]
	pub install_telemetry_info: Option<InstallTelemetryInfo>,
	#[serde(rename = "recommendationState")]
	pub recommendation_state: Option<RecommendationState>,
	#[serde(rename = "rootURI")]
	pub root_uri: Option<String>,
	pub location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultLocale {
	pub contributors: Option<String>,
	pub creator: Option<String>,
	pub description: Option<String>,
	pub developers: Option<String>,
	pub name: Option<String>,
	pub translators: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Locale {
	pub contributors: Option<String>,
	pub creator: Option<String>,
	pub description: Option<String>,
	pub developers: Option<String>,
	pub locales: Option<Vec<String>>,
	pub name: Option<String>,
	pub translators: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permissions {
	pub permissions: Vec<String>,
	pub origins: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetApplication {
	pub id: String,
	#[serde(rename = "minVersion")]
	pub min_version: Option<String>,
	#[serde(rename = "maxVersion")]
	pub max_version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecommendationState {
	#[serde(rename = "validNotAfter")]
	pub valid_not_after: u64,
	#[serde(rename = "validNotBefore")]
	pub valid_not_before: u64,
	pub states: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocaleFile {
	pub extension_description: Message,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
	pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstallTelemetryInfo {
	pub source: String,
	pub method: String,
}
