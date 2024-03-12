use super::browser_specific_settings::BrowserSpecificSettings;
use crate::configuration::profile::Profile;
use crate::errors::Error;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::extensions_json_database::addon::addon::Addon;
use crate::extensions_json_database::addon::default_locale::DefaultLocale;
use crate::extensions_json_database::addon::install_telemetry_info::InstallTelemetryInfo;
use crate::extensions_json_database::addon::locale::Locale;
use crate::extensions_json_database::addon::permissions::Permissions;
use crate::extensions_json_database::addon::recommendation_state::RecommendationState;
use crate::extensions_json_database::addon::target_application::TargetApplication;
use crate::extensions_json_database::extensions_json_database::ExtensionsJsonDatabase;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use zip::ZipArchive;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
	pub manifest_version: u8,
	pub name: String,
	pub version: String,
	pub author: Option<String>,
	pub description: Option<String>,
	pub default_locale: String,
	pub icons: HashMap<String, String>,
	pub permissions: Vec<String>,
	pub optional_permissions: Option<Vec<String>>,
	pub browser_specific_settings: BrowserSpecificSettings,
}

impl Manifest {
	fn read_locales(manifest: &Manifest, zip: &mut ZipArchive<File>) -> Result<Vec<Locale>> {
		let range = 0..zip.len();
		let locale_folders: Vec<String> = range
			.into_iter()
			.filter_map(|index| {
				let file = zip.by_index(index).unwrap();
				if file.name().starts_with("_locales") {
					Some(file.name().to_owned())
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
					.map(|o| o.to_owned())
					.collect::<Vec<String>>()
					.get(1)
					.unwrap()
					.to_owned();
				let mut messages_json = zip.by_name(locale).unwrap();
				let mut content = String::new();
				messages_json.read_to_string(&mut content).unwrap();
				// FIX: below
				// let locale_file: LocaleFile = serde_json::from_str(content.as_str())?;
				Locale {
					description: None, // Some(locale_file.extension_description.unwrap().message),
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

	fn generate_addon_info(
		manifest: &Manifest,
		locales: Vec<Locale>,
		path: &Path,
	) -> Result<Addon> {
		let target_applications = TargetApplication::try_from(manifest)?;
		let user_permissions = Permissions::try_from(manifest)?;
		let optional_permissions = Permissions::default();
		let recommendation_state = RecommendationState::new();
		let install_telemetry_info = InstallTelemetryInfo::new();
		let install_date = chrono::Utc::now().timestamp_millis();
		let update_date = install_date;

		let description = locales
			.iter()
			.find(|l| l.locales.clone().unwrap()[0] == manifest.default_locale.clone())
			.unwrap()
			.description
			.clone();

		let default_locale = DefaultLocale {
			name: Some(manifest.name.clone()),
			description,
			creator: manifest.author.clone(),
			contributors: None,
			developers: None,
			translators: None,
		};

		let addon = Addon {
			ty: String::from("extension"),
			about_url: None,
			active: true,
			app_disabled: false,
			apply_background_updates: 1,
			blocklist_state: Some(0),
			blocklist_url: None,
			default_locale: Some(default_locale),
			dependencies: Vec::new(),
			embedder_disabled: false,
			foreign_install: true,
			hidden: Some(false),
			icon_url: None,
			icons: manifest.icons.clone(),
			id: manifest.browser_specific_settings.gecko.id.clone(),
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
			path: Some(path.to_path_buf()),
			recommendation_state: Some(recommendation_state),
			release_notes_uri: None,
			root_uri: Some(format!("jar:file://{}!/", path.display())),
			seen: Some(true),
			signed_date: None,
			signed_state: Some(2),
			skinnable: false,
			soft_disabled: false,
			source_uri: None,
			// startup_data: None,
			strict_compatibility: true,
			sync_guid: Some(String::new()),
			target_applications: vec![target_applications],
			target_platforms: Vec::new(),
			update_date: Some(update_date),
			update_url: None,
			user_disabled: false,
			user_permissions: Some(user_permissions),
			version: manifest.version.clone(),
			visible: true,
		};

		Ok(addon)
	}

	fn get_addon(path: &Path, ext: &Extension) -> Result<Addon> {
		let ext_path = PathBuf::from(format!(
			"{}.xpi",
			path.join("extensions").join(&ext.guid).display()
		));
		let ext_file = std::fs::File::open(&ext_path)?;
		let mut zip = zip::ZipArchive::new(ext_file).unwrap();
		let manifest = Self::try_from(&mut zip)?;
		let locales = Self::read_locales(&manifest, &mut zip)?;
		let addon = Self::generate_addon_info(&manifest, locales, &ext_path)?;
		Ok(addon)
	}

	pub fn add_extension_to_database(profile: &Profile, ext: &Extension) -> Result<()> {
		let mut addons = ExtensionsJsonDatabase::try_from(profile)?;
		let addon = Self::get_addon(&profile.path, ext)?;
		addons.addons.push(addon);
		let content = serde_json::to_string(&addons)?;
		std::fs::write(&profile.path.join("extensions.json"), content)?;

		Ok(())
	}
}

impl TryFrom<&mut ZipArchive<File>> for Manifest {
	type Error = Error;

	fn try_from(zip: &mut ZipArchive<File>) -> Result<Self> {
		let mut manifest_file = zip.by_name("manifest.json").unwrap();
		let mut content = String::new();
		manifest_file.read_to_string(&mut content).unwrap();
		let manifest: Manifest = serde_json::from_str(content.as_str())?;
		Ok(manifest)
	}
}
