use super::default_locale::DefaultLocale;
use super::install_telemetry_info::InstallTelemetryInfo;
use super::locales::Locales;
use super::permissions::Permissions;
use super::recommendation_state::RecommendationState;
use super::startup_data::StartupData;
use super::target_application::TargetApplication;
use crate::addon::addon::Addon;
use crate::configuration::profile::Profile;
use crate::database::manifests::manifest::Manifest;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsJsonAddon {
	#[serde(rename = "aboutURL")]
	about_url: Option<Url>,
	active: bool,
	app_disabled: bool,
	apply_background_updates: u32,
	blocklist_state: Option<u32>,
	#[serde(rename = "blocklistURL")]
	blocklist_url: Option<Url>,
	default_locale: Option<DefaultLocale>,
	dependencies: Vec<String>,
	embedder_disabled: bool,
	foreign_install: bool,
	hidden: Option<bool>,
	#[serde(rename = "iconURL")]
	icon_url: Option<String>,
	icons: HashMap<String, String>,
	pub id: String,
	incognito: Option<String>,
	install_date: Option<u64>,
	install_origins: Option<String>,
	install_telemetry_info: Option<InstallTelemetryInfo>,
	loader: Option<String>,
	locales: Locales,
	location: String,
	manifest_version: u8,
	optional_permissions: Option<Permissions>,
	options_browser_style: bool,
	options_type: Option<u32>,
	options_url: Option<Url>,
	pub path: Option<PathBuf>,
	recommendation_state: Option<RecommendationState>,
	#[serde(rename = "releaseNotesURI")]
	release_notes_uri: Option<Url>,
	#[serde(rename = "rootURI")]
	root_uri: Option<Url>,
	seen: Option<bool>,
	signed_date: Option<u64>,
	signed_state: Option<u8>,
	skinnable: bool,
	soft_disabled: bool,
	#[serde(rename = "sourceURI")]
	source_uri: Option<Url>,
	startup_data: Option<StartupData>,
	strict_compatibility: bool,
	#[serde(rename = "syncGUID")]
	pub sync_guid: Option<String>,
	target_applications: Vec<TargetApplication>,
	target_platforms: Vec<String>,
	#[serde(rename = "type")]
	ty: String,
	update_date: Option<u64>,
	update_url: Option<Url>,
	user_disabled: bool,
	user_permissions: Option<Permissions>,
	version: String,
	visible: bool,
}

impl TryFrom<(&(&Addon, Vec<u8>), &Manifest, &Profile)> for ExtensionsJsonAddon {
	type Error = anyhow::Error;

	fn try_from(value: (&(&Addon, Vec<u8>), &Manifest, &Profile)) -> Result<Self> {
		let addon = value.0 .0;
		let bytes = &value.0 .1;
		let manifest = value.1;
		let profile = value.2;

		let target_applications = TargetApplication::try_from(manifest)?;
		let user_permissions = Permissions::try_from(addon)?;
		let optional_permissions = Permissions::try_from(addon)?;
		let recommendation_state = RecommendationState::new(); // FIX:
		let install_telemetry_info = InstallTelemetryInfo::new(); // FIX:
		let install_date = chrono::Utc::now().timestamp_millis() as u64;
		let update_date = install_date;
		let locales = Locales::try_from((bytes, manifest))?;
		let default_locale = DefaultLocale::try_from(manifest)?;
		let path = profile.extensions.join(format!("{}.xpi", &addon.guid));

		let addon = Self {
			about_url: None,
			active: true,
			app_disabled: false,
			apply_background_updates: 1,
			blocklist_state: Some(0),
			blocklist_url: None,
			default_locale: Some(default_locale),
			dependencies: Vec::default(),
			embedder_disabled: false,
			foreign_install: true,
			hidden: Some(false),
			icon_url: None,
			icons: manifest.icons.clone(),
			id: addon.guid.clone(),
			incognito: Some(String::from("spanning")),
			install_date: Some(install_date),
			install_origins: None,
			install_telemetry_info: Some(install_telemetry_info),
			loader: None,
			locales,
			location: String::from("app-profile"),
			manifest_version: manifest.manifest_version,
			optional_permissions: Some(optional_permissions),
			options_browser_style: true,
			options_type: None,
			options_url: None,
			path: Some(path.to_owned()),
			recommendation_state: Some(recommendation_state),
			release_notes_uri: None,
			root_uri: None,
			seen: Some(true),
			signed_date: None,
			signed_state: None,
			skinnable: false,
			soft_disabled: false,
			source_uri: None,
			startup_data: None,
			strict_compatibility: true,
			sync_guid: None,
			target_applications: vec![target_applications],
			target_platforms: Vec::default(),
			ty: String::from("extension"),
			update_date: Some(update_date),
			update_url: None,
			user_disabled: false,
			user_permissions: Some(user_permissions),
			version: manifest.version.to_owned(),
			visible: true,
		};

		Ok(addon)
	}
}

impl ExtensionsJsonAddon {
	pub fn is_not_builtin(&self) -> bool {
		self.location != "app-builtin" && self.location != "app-system-defaults"
	}
}

impl Display for ExtensionsJsonAddon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.id)
	}
}
