use clap::Parser;
use ini::Ini;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::copy;
use thiserror::Error;

const QUERY_URL: &str = "https://addons.mozilla.org/api/v5/addons/search/?q=";
const DOWNLOAD_URL: &str = "https://addons.mozilla.org/firefox/downloads/file";

// clap
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long, num_args = 1.., value_delimiter = ' ')]
	install: Vec<String>,

	#[arg(short, long, default_value = "default-release")]
	profile: String,

	#[arg(short, long, default_value = "firefox")]
	browser: String,
}

impl Args {
	async fn get_profiles(&self) -> Result<Vec<Profile>, ProfileError> {
		let browser_data_path = self.get_browser_data_path().await?;
		let ini = format!("{}/profiles.ini", &browser_data_path);
		let config = Ini::load_from_file(ini)?;
		let profiles: Vec<Profile> = config
			.iter()
			.filter_map(|(sector, property)| {
				sector.and_then(|sec| {
					if sec.starts_with("Profile") {
						let name = property.get("Name")?.to_string();
						let path = property.get("Path")?.to_string();
						Some(Profile::from(name, path))
					} else {
						None
					}
				})
			})
			.collect();

		Ok(profiles)
	}

	async fn validate_args(self) -> Result<Self, ArgsError> {
		let profiles = self.get_profiles().await?;
		let names: Vec<String> = profiles.iter().map(|profile| profile.name.clone()).collect();
		if !names.contains(&self.profile) {
			eprintln!("Profile {} does not exist.", &self.profile);
			return Err(ArgsError::ProfileNotFound);
		}

		Ok(self)
	}

	async fn get_browser_data_path(&self) -> Result<String, std::env::VarError> {
		let home = std::env::var("HOME")?;
		let mut path = String::new();

		if &self.browser == "firefox" {
			path = format!("{}/.mozilla/{}", home, &self.browser);
		} else if &self.browser == "librewolf" {
			path = format!("{}/.{}", home, &self.browser);
		}

		Ok(path)
	}

	async fn get_download_path(&self) -> Result<String, ProfileError> {
		let browser_data_path = &self.get_browser_data_path().await?;
		let profiles = self.get_profiles().await?;
		let profile: &Profile = profiles.iter().find(|p| p.name == self.profile).unwrap();
		Ok(format!("{}/{}/extensions", &browser_data_path, &profile.path))
	}
}

// serde
#[derive(Debug, Serialize, Deserialize)]
struct FileCurrentVersion {
	id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentVersion {
	file: FileCurrentVersion,
}

#[derive(Debug, Serialize, Deserialize)]
struct Extension {
	id: i32,
	slug: String,
	guid: String,
	current_version: CurrentVersion,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryResult {
	results: Vec<Extension>,
}

// thiserror
#[derive(Debug, Error)]
enum ProfileError {
	#[error("Can not find $HOME enviornment variable.")]
	HomeVar(#[from] std::env::VarError),

	#[error("Ini error.")]
	Ini(#[from] ini::Error),
}

#[derive(Debug, Error)]
enum InstallError {
	#[error("Request error.")]
	Request(#[from] reqwest::Error),

	#[error("Profile path error.")]
	Profile(#[from] ProfileError),

	#[error("Creating file error.")]
	CreateFile(#[from] std::io::Error),
}

#[derive(Debug, Error)]
enum ArgsError {
	#[error("Profile error.")]
	Profile(#[from] ProfileError),

	#[error("Specified profile does not exist.")]
	ProfileNotFound,
}

// profiles
#[derive(Debug, Default)]
struct Profile {
	name: String,
	path: String,
}

impl Profile {
	fn from(name: String, path: String) -> Self {
		Self { name, path }
	}
}

async fn query_extension(extension: &str) -> Result<QueryResult, reqwest::Error> {
	let query_request = reqwest::Client::new().get(format!("{}{}", QUERY_URL, extension)).send().await?.json().await?;
	Ok(query_request)
}

async fn install_extension(extension: &Extension, download_path: &String) -> Result<(), InstallError> {
	let ext_guid = &extension.guid;
	let ext_ver = &extension.current_version.file.id;
	let request = reqwest::get(format!("{}/{}", DOWNLOAD_URL, &ext_ver)).await?;
	if request.status().is_success() {
		let ext_path = format!("{}/{}.xpi", &download_path, &ext_guid);
		let mut file = File::create(&ext_path)?;
		copy(&mut request.bytes().await?.as_ref(), &mut file)?;
	}

	Ok(())
}

async fn install_extensions(extensions: &[String], download_path: &String) -> Result<(), InstallError> {
	for ext in extensions {
		let query_result = query_extension(ext).await?;
		if let Some(extension) = query_result.results.first() {
			install_extension(extension, download_path).await?;
		} else {
			eprintln!("Extension not found: {}", ext);
		}
	}

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ArgsError> {
	let args = Args::parse().validate_args().await?;
	let download_path = args.get_download_path().await?;

	if !args.install.is_empty() {
		match install_extensions(&args.install, &download_path).await {
			Ok(_) => println!("Successfully installed extension"),
			Err(err) => eprintln!("Error installing extension: {err}"),
		}
	}

	Ok(())
}
