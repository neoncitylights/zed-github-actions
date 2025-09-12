use std::collections::HashSet;
use zed_extension_api::*;

const PACKAGE_NAME: &str = "gh-actions-language-server";
const BINARY_NAME: &str = PACKAGE_NAME;

struct GitHubActionsExtension {
	installed: HashSet<String>,
}

impl GitHubActionsExtension {
	fn binary_path() -> String {
		std::env::current_dir()
			.unwrap()
			.join("node_modules")
			.join(PACKAGE_NAME)
			.join(format!("bin/{BINARY_NAME}"))
			.to_string_lossy()
			.to_string()
	}

	fn install_package_if_needed(
		&mut self,
		id: &LanguageServerId,
		package_name: &str,
	) -> Result<()> {
		use LanguageServerInstallationStatus::*;
		let installed_version = npm_package_installed_version(package_name)?;

		// If package is already installed in this session, then we won't reinstall it
		if installed_version.is_some() && self.installed.contains(package_name) {
			return Ok(());
		}

		set_language_server_installation_status(id, &CheckingForUpdate);

		let latest_version = npm_package_latest_version(package_name)?;

		if installed_version.as_ref() != Some(&latest_version) {
			println!("Installing {package_name}@{latest_version}...");

			set_language_server_installation_status(id, &Downloading);

			if let Err(error) = npm_install_package(package_name, &latest_version) {
				// If installation failed, but we don't want to error but rather reuse existing version
				if installed_version.is_none() {
					Err(error)?;
				}
			}
		} else {
			println!("Found {package_name}@{latest_version} installed");
		}

		self.installed.insert(package_name.into());
		Ok(())
	}
}

impl Extension for GitHubActionsExtension {
	fn new() -> Self {
		Self {
			installed: HashSet::new(),
		}
	}

	fn language_server_command(
		&mut self,
		language_server_id: &LanguageServerId,
		_worktree: &Worktree,
	) -> Result<Command> {
		self.install_package_if_needed(language_server_id, PACKAGE_NAME)?;

		Ok(Command {
			command: node_binary_path()?,
			args: vec![Self::binary_path(), "--stdio".to_string()],
			env: Default::default(),
		})
	}

	fn language_server_initialization_options(
		&mut self,
		_language_server_id: &LanguageServerId,
		_worktree: &Worktree,
	) -> Result<Option<serde_json::Value>> {
		Ok(Some(serde_json::json!({
			"sessionToken": ""
		})))
	}
}

register_extension!(GitHubActionsExtension);
