use crate::addon::Addon;
use crate::addon::Addons;
use crate::addon::DefaultLocale;
use crate::addon::Locale;
use crate::addon::LocaleFile;
use crate::addon::Permissions;
use crate::addon::RecommendationState;
use crate::addon::TargetApplication;
use crate::errors::Result;
use crate::extension::extension::Extension;
use crate::flags::flags::Flags;
use crate::manifest::Manifest;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

async fn read_manifest(flags: &Flags, ext: &Extension) -> Result<Addon> {
	let ext_path = PathBuf::from(format!(
		"{}.xpi",
		&flags
			.profile
			.path
			.join("extensions")
			.join(&ext.guid)
			.display()
	));

	let file = std::fs::File::open(ext_path)?;
	let mut zip = zip::ZipArchive::new(&file).unwrap();
	let mut manifest_file = zip
		.by_name("manifest.json")
		.or(Err(crate::errors::Error::Home))?;
	let mut content = String::new();
	manifest_file.read_to_string(&mut content).unwrap();
	let manifest: Manifest = serde_json::from_str(content.as_str())?;

	let path = PathBuf::from(format!("{}/{}.xpi", flags.profile.path.display(), ext.guid));
	let target_applications = TargetApplication {
		id: String::from("toolkit@mozilla.org"),
		min_version: manifest
			.clone()
			.browser_specific_settings
			.gecko
			.strict_min_version,
		max_version: manifest
			.clone()
			.browser_specific_settings
			.gecko
			.strict_max_version,
	};
	let user_permissions = Permissions {
		permissions: manifest.clone().permissions,
		origins: manifest
			.clone()
			.permissions
			.iter()
			.filter(|o| o.starts_with("<"))
			.map(|o| o.clone())
			.collect(),
	};
	let optional_permissions = Permissions { permissions: Vec::new(), origins: Vec::new() };

	// locales
	let mut zip = zip::ZipArchive::new(&file).unwrap();
	let mut locale_folders: Vec<String> = Vec::new();

	for i in 0..zip.len() {
		let f = zip.by_index(i).unwrap();
		if f.name().starts_with("_locales") {
			locale_folders.push(f.name().to_owned());
		}
	}

	let mut locales: Vec<Locale> = Vec::new();
	for l in locale_folders {
		let locale_slug = l
			.split("/")
			.collect::<Vec<&str>>()
			.get(1)
			.unwrap()
			.to_string();
		let mut messages_json = zip
			.by_name(l.as_str())
			.or(Err(crate::errors::Error::Home))?;
		let mut content = String::new();
		messages_json.read_to_string(&mut content).unwrap();
		let l_f: LocaleFile = serde_json::from_str(content.as_str())?;
		let locale = Locale {
			description: Some(l_f.extension_description.message),
			locales: Some(vec![locale_slug]),
			contributors: None,
			translators: None,
			creator: Some(manifest.clone().author),
			developers: None,
			name: Some(manifest.clone().name),
		};
		locales.push(locale);
	}

	let asd = locales
		.iter()
		.find(|l| l.locales.iter().find(|a| a.as_ow == "en").unwrap())
		.unwrap();

	let default_locale = DefaultLocale {
		name: Some(manifest.clone().name),
		description: Some(),
		creator: Some(manifest.clone().author),
		contributors: None,
		developers: None,
		translators: None,
	};

	let recommendation_state = RecommendationState {
		valid_not_after: 0,
		valid_not_before: 0,
		states: vec!["recommended-android".to_string(), "recommended".to_string()],
	};

	let addon = Addon {
		_type: String::from("extension"),
		about_url: None,
		active: true,
		app_disabled: false,
		apply_background_updates: 1,
		blocklist_state: Some(0),
		blocklist_url: None,
		default_locale: Some(default_locale),
		dependencies: Vec::new(),
		embedder_disabled: false,
		foreign_install: false,
		hidden: Some(false),
		icon_url: None,
		icons: manifest.clone().icons.clone(),
		id: manifest.clone().browser_specific_settings.gecko.id,
		incognito: Some(String::from("spanning")),
		install_date: None,
		install_origins: None,
		install_telemetry_info: None,
		loader: None,
		locales,
		location: Some(String::from("app-builtin")),
		manifest_version: manifest.manifest_version,
		optional_permissions: Some(optional_permissions),
		options_browser_style: true,
		options_type: None,
		options_url: None,
		path: Some(path.clone()),
		recommendation_state: Some(recommendation_state),
		release_notes_uri: None,
		root_uri: Some(format!("jar:file://{}!/", path.clone().display())),
		seen: Some(true),
		signed_date: None,
		signed_state: Some(2),
		skinnable: false,
		soft_disabled: false,
		source_uri: None,
		startup_data: None,
		strict_compatibility: true,
		sync_guid: Some(String::new()),
		target_applications: vec![target_applications],
		target_platforms: Vec::new(),
		update_date: None,
		update_url: None,
		user_disabled: false,
		user_permissions: Some(user_permissions),
		version: manifest.version,
		visible: true,
	};

	Ok(addon)
}

pub async fn add_extension(flags: &Flags) -> Result<()> {
	let path = flags.profile.path.join("extensions.json");

	if !path.exists() {
		tokio::fs::File::create(&path).await?;
	}

	let file = std::fs::File::open(&path)?;
	let reader = BufReader::new(&file);
	let mut addons: Addons = serde_json::from_reader(reader)?;

	for ext in &flags.install.extensions {
		let addon = read_manifest(&flags, &ext).await?;
		addons.addons.push(addon);
	}

	let content = serde_json::to_string(&addons)?;
	tokio::fs::write(path, content).await?;

	Ok(())
}