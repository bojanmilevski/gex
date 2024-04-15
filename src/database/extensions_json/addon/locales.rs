use crate::database::manifests::manifest::Manifest;
use crate::errors::Error;
use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;
use std::io::Cursor;
use std::io::Read;
use zip::ZipArchive;

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

impl TryFrom<(&Vec<u8>, &Manifest)> for Locales {
	type Error = Error;

	fn try_from(value: (&Vec<u8>, &Manifest)) -> Result<Self> {
		let bytes = value.0;
		let manifest = value.1;

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

		Ok(Self { locales })
	}
}
