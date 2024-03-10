use super::default_locale::DefaultLocale;
use super::install_telemetry_info::InstallTelemetryInfo;
use super::locale::Locale;
use super::permissions::Permissions;
use super::recommendation_state::RecommendationState;
use super::target_application::TargetApplication;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Addon {
	pub id: String,
	pub sync_guid: Option<String>,
	pub version: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub loader: Option<String>,
	pub update_url: Option<String>,
	pub install_origins: Option<String>,
	pub manifest_version: u8,
	pub options_url: Option<String>,
	pub options_type: Option<u32>,
	pub options_browser_style: bool,
	#[serde(rename = "aboutURL")]
	pub about_url: Option<String>,
	pub default_locale: Option<DefaultLocale>,
	pub visible: bool,
	pub active: bool,
	pub user_disabled: bool,
	pub app_disabled: bool,
	pub embedder_disabled: bool,
	pub install_date: Option<i64>,
	pub update_date: Option<i64>,
	pub apply_background_updates: u32,
	pub path: Option<PathBuf>,
	pub skinnable: bool,
	#[serde(rename = "sourceURI")]
	pub source_uri: Option<String>,
	#[serde(rename = "releaseNotesURI")]
	pub release_notes_uri: Option<String>,
	pub soft_disabled: bool,
	pub foreign_install: bool,
	pub strict_compatibility: bool,
	pub locales: Vec<Locale>,
	pub target_applications: Vec<TargetApplication>,
	pub target_platforms: Vec<String>,
	pub signed_state: Option<u8>,
	pub signed_date: Option<u64>,
	pub seen: Option<bool>,
	pub dependencies: Vec<String>,
	pub incognito: Option<String>,
	pub user_permissions: Option<Permissions>,
	pub optional_permissions: Option<Permissions>,
	pub icons: HashMap<String, String>,
	#[serde(rename = "iconURL")]
	pub icon_url: Option<String>,
	pub blocklist_state: Option<u32>,
	#[serde(rename = "blocklistURL")]
	pub blocklist_url: Option<String>,
	// FIX: below
	// pub startup_data: Option<HashMap<String, String>>,
	pub hidden: Option<bool>,
	pub install_telemetry_info: Option<InstallTelemetryInfo>,
	pub recommendation_state: Option<RecommendationState>,
	#[serde(rename = "rootURI")]
	pub root_uri: Option<String>,
	pub location: String,
}

impl Display for Addon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.id)
	}
}
