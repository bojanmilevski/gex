use super::default_locale::DefaultLocale;
use super::install_telemetry_info::InstallTelemetryInfo;
use super::locale::Locale;
use super::permissions::Permissions;
use super::recommendation_state::RecommendationState;
use super::target_application::TargetApplication;
use crate::configuration::profile::Profile;
use crate::database::manifests::manifest::Manifest;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;
use std::path::PathBuf;
use url::Url;
use zip::ZipArchive;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionsJsonAddon {
	#[serde(rename = "aboutURL")]
	pub about_url: Option<Url>,
	pub active: bool,
	pub app_disabled: bool,
	pub apply_background_updates: u32,
	pub blocklist_state: Option<u32>,
	#[serde(rename = "blocklistURL")]
	pub blocklist_url: Option<Url>,
	pub default_locale: Option<DefaultLocale>,
	pub dependencies: Vec<String>,
	pub embedder_disabled: bool,
	pub foreign_install: bool,
	pub hidden: Option<bool>,
	#[serde(rename = "iconURL")]
	pub icon_url: Option<String>,
	pub icons: HashMap<String, String>,
	pub id: String,
	pub incognito: Option<String>,
	pub install_date: Option<i64>,
	pub install_origins: Option<String>,
	pub install_telemetry_info: Option<InstallTelemetryInfo>,
	pub loader: Option<String>,
	pub locales: Vec<Locale>,
	pub location: String,
	pub manifest_version: u8,
	pub optional_permissions: Option<Permissions>,
	pub options_browser_style: bool,
	pub options_type: Option<u32>,
	pub options_url: Option<Url>,
	pub path: Option<PathBuf>,
	pub recommendation_state: Option<RecommendationState>,
	#[serde(rename = "releaseNotesURI")]
	pub release_notes_uri: Option<Url>,
	#[serde(rename = "rootURI")]
	pub root_uri: Option<Url>,
	pub seen: Option<bool>,
	pub signed_date: Option<u64>,
	pub signed_state: Option<u8>,
	pub skinnable: bool,
	pub soft_disabled: bool,
	#[serde(rename = "sourceURI")]
	pub source_uri: Option<Url>,
	pub startup_data: Option<StartupData>,
	pub strict_compatibility: bool,
	pub sync_guid: Option<String>,
	pub target_applications: Vec<TargetApplication>,
	pub target_platforms: Vec<String>,
	#[serde(rename = "type")]
	pub ty: String,
	pub update_date: Option<i64>,
	pub update_url: Option<Url>,
	pub user_disabled: bool,
	pub user_permissions: Option<Permissions>,
	pub version: String,
	pub visible: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupData {
	persistent_listeners: PersistentListeners,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentListeners {
	web_request: WebRequest,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebRequest {
	on_before_request: Vec<Vec<Request>>,
	on_before_send_headers: Vec<Vec<Request>>,
	on_headers_received: Vec<Vec<Request>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
	incognito: Option<bool>,
	tab_id: Option<u32>,
	types: Vec<String>,
	urls: Vec<String>, // TODO:
	window_id: Option<u32>,
	#[serde(default)]
	actions: Vec<String>,
}

impl TryFrom<(&Vec<u8>, &Manifest, &Profile)> for ExtensionsJsonAddon {
	type Error = Error;

	fn try_from(value: (&Vec<u8>, &Manifest, &Profile)) -> Result<Self> {
		let target_applications = TargetApplication::try_from(value.1)?;
		let user_permissions = Permissions::try_from(value.1)?;
		let optional_permissions = Permissions::default();
		let recommendation_state = RecommendationState::new();
		let install_telemetry_info = InstallTelemetryInfo::new();
		let install_date = chrono::Utc::now().timestamp_millis();
		let update_date = install_date;
		let locales = Self::get_locales(value.0, value.1)?;

		let description = locales
			.iter()
			.find(|l| l.locales.clone().unwrap()[0] == value.1.default_locale.clone().unwrap())
			.unwrap()
			.description
			.clone();

		let default_locale = DefaultLocale {
			contributors: None,
			creator: value.1.author.clone(),
			description,
			developers: None,
			homepage_url: None,
			name: Some(value.1.name.clone()),
			translators: None,
		};

		let path = value
			.2
			.path
			.join("extensions")
			.join(format!("{}.xpi", &value.1.browser_specific_settings.gecko.id));

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
			icons: value.1.icons.clone(),
			id: value.1.browser_specific_settings.gecko.id.clone(),
			incognito: Some(String::from("spanning")),
			install_date: Some(install_date),
			install_origins: None,
			install_telemetry_info: Some(install_telemetry_info),
			loader: None,
			locales,
			location: String::from("app-profile"),
			manifest_version: value.1.manifest_version,
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
			version: value.1.version.to_owned(),
			visible: true,
		};

		Ok(addon)
	}
}

impl ExtensionsJsonAddon {
	fn get_locales(bytes: &Vec<u8>, manifest: &Manifest) -> Result<Vec<Locale>> {
		let cursor = Cursor::new(bytes);
		let mut zip = ZipArchive::new(cursor).unwrap();
		let range = 0..zip.len();
		let locale_folders: Vec<String> = range
			.into_iter()
			.filter_map(|index| {
				let file = zip.by_index(index).unwrap();
				if file.name().starts_with("_locales") {
					Some(String::from(file.name()))
				} else {
					None
				}
			})
			.collect();

		let locales = locale_folders
			.iter()
			.map(|locale| {
				let locale_slug = locale
					.split('/')
					.collect::<Vec<&str>>()
					.get(1)
					.unwrap()
					.to_owned();
				let mut messages_json = zip.by_name(locale).unwrap();
				let mut content = String::new();
				messages_json.read_to_string(&mut content).unwrap();
				// let locale_file: LocaleFile = serde_json::from_str(content.as_str())?; // FIX:
				Locale {
					description: None, // Some(locale_file.extension_description.unwrap().message), // FIX:
					locales: Some(vec![locale_slug.replace('_', "-")]),
					contributors: None,
					translators: None,
					creator: manifest.author.clone(),
					developers: None,
					name: Some(manifest.name.clone()),
				}
			})
			.collect();

		Ok(locales)
	}

	pub fn is_not_builtin(&self) -> bool {
		self.location != "app-builtin" && self.location != "app-system-defaults"
	}
}

impl Display for ExtensionsJsonAddon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.id)
	}
}
