use crate::addon::Addon;
use crate::database::manifests::manifest::Manifest;
use crate::operation::install::Package;
use crate::profile::Profile;
use anyhow::Context;
use anyhow::Result;
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

impl<'a> TryFrom<(&Package<'a>, &Profile)> for ExtensionsJsonAddon {
	type Error = anyhow::Error;

	fn try_from(value: (&Package, &Profile)) -> Result<Self> {
		let addon = value.0.json_response;
		let bytes = &value.0.xpi;
		let manifest = &value.0.manifest;
		let profile = value.1;

		let target_applications = TargetApplication::try_from(manifest)?;
		let user_permissions = Permissions::try_from(addon)?;
		let optional_permissions = Permissions::try_from(addon)?;
		let recommendation_state = RecommendationState::new(); // FIX:
		let install_telemetry_info = InstallTelemetryInfo::new(); // FIX:
		let install_date = chrono::Utc::now().timestamp_millis() as u64;
		let update_date = install_date;
		let locales = Locales::try_from((bytes.as_ref(), manifest))?;
		let default_locale = DefaultLocale::try_from(manifest)?;
		let path = profile
			.path
			.join("extensions")
			.join(format!("{}.xpi", &addon.guid));
		let uuid = uuid::Uuid::new_v4();
		let sync_guid = Some(format!("{{{}}}", uuid));

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
			icons: manifest.icons.clone().unwrap(),
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
			sync_guid,
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

	pub fn slug(&self) -> String {
		String::new()
	}
}

impl Display for ExtensionsJsonAddon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.id)
	}
}

#[derive(Serialize, Deserialize)]
pub struct DefaultLocale {
	contributors: Option<String>,
	creator: Option<String>,
	description: Option<String>,
	developers: Option<String>,
	#[serde(rename = "homepageURL")]
	homepage_url: Option<Url>,
	name: Option<String>,
	translators: Option<String>,
}

impl TryFrom<&Manifest> for DefaultLocale {
	type Error = anyhow::Error;

	fn try_from(manifest: &Manifest) -> Result<Self> {
		let default_locale = Self {
			contributors: None,
			creator: manifest.author.clone(),
			description: None,
			developers: None,
			homepage_url: None,
			name: Some(manifest.name.clone()),
			translators: None,
		};

		Ok(default_locale)
	}
}

#[derive(Serialize, Deserialize)]
pub struct InstallTelemetryInfo {
	pub method: Option<String>,
	pub source: Option<String>,
	source_url: Option<Url>,
}

impl InstallTelemetryInfo {
	pub fn new() -> Self {
		Self { source: Some(String::from("app-profile")), method: Some(String::from("sideload")), source_url: None }
	}
}

#[derive(Serialize, Deserialize)]
pub struct LocaleFile {
	pub extension_description: Option<Message>,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Locales {
	pub locales: Vec<Locale>,
}

#[derive(Serialize, Deserialize)]
pub struct Locale {
	contributors: Option<String>,
	creator: Option<String>,
	pub description: Option<String>,
	developers: Option<String>,
	pub locales: Option<Vec<String>>,
	name: Option<String>,
	translators: Option<String>,
}

impl TryFrom<(&[u8], &Manifest)> for Locales {
	type Error = anyhow::Error;

	fn try_from(value: (&[u8], &Manifest)) -> Result<Self> {
		let bytes = value.0;
		let manifest = value.1;

		let cursor = Cursor::new(bytes);
		let mut zip = ZipArchive::new(cursor).context("Cannot open .xpi file.")?;
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
					description: None, // Some(locale_file.extension_description.context("No extension_description.").message), // FIX:
					locales: Some(vec![locale_slug.replace('_', "-")]),
					contributors: None,
					translators: None,
					creator: manifest.author.clone(),
					developers: None,
					name: Some(manifest.name.clone()),
				}
			})
			.collect();

		Ok(Self { locales })
	}
}

#[derive(Serialize, Deserialize)]
pub struct Message {
	pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Permissions {
	pub origins: Vec<String>,
	pub permissions: Vec<String>,
}

impl TryFrom<&Addon> for Permissions {
	type Error = anyhow::Error;

	fn try_from(addon: &Addon) -> Result<Self> {
		let permissions = addon
			.current_version
			.file
			.permissions
			.iter()
			.filter(|addon| !addon.contains(".com")) // FIX: this is not right
			.cloned()
			.collect();

		let origins = addon
			.current_version
			.file
			.permissions
			.iter()
			.filter(|addon| addon.contains(".com")) // FIX: this is not right
			.cloned()
			.collect();

		Ok(Self { origins, permissions })
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentListeners {
	web_request: Option<WebRequest>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationState {
	pub states: Vec<String>,
	pub valid_not_after: u64,
	pub valid_not_before: u64,
}

impl RecommendationState {
	pub fn new() -> Self {
		Self {
			states: vec![String::from("recommended-android"), String::from("recommended")],
			valid_not_after: 0,
			valid_not_before: 0,
		}
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
	incognito: Option<bool>,
	tab_id: Option<u32>,
	types: Option<Vec<String>>,
	urls: Vec<String>, // FIX: Url
	window_id: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupData {
	persistent_listeners: Option<PersistentListeners>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetApplication {
	pub id: String,
	pub max_version: Option<String>,
	pub min_version: Option<String>,
}

impl TryFrom<&Manifest> for TargetApplication {
	type Error = anyhow::Error;

	fn try_from(manifest: &Manifest) -> Result<Self> {
		let id = String::from("toolkit@mozilla.org");
		let min_version = None;
		let max_version = None;

		Ok(Self { id, min_version, max_version })
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebRequest {
	on_before_request: Option<Vec<(Request, Vec<String>)>>,
	on_before_send_headers: Option<Vec<(Request, Vec<String>)>>,
	on_headers_received: Option<Vec<(Request, Vec<String>)>>,
}
