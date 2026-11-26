use std::collections::HashSet;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::*;

const PACKAGE_NAME: &str = "@actions/languageserver";
const BINARY_NAME: &str = "actions-languageserver";

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

/// Runs a program from the worktree's `$PATH`, returning its trimmed stdout if it succeeds
fn run(worktree: &Worktree, program: &str, args: &[&str]) -> Option<String> {
	let output = process::Command::new(worktree.which(program)?)
		.args(args.iter().copied())
		.envs(worktree.shell_env())
		.output()
		.ok()?;
	if output.status != Some(0) {
		return None;
	}
	String::from_utf8(output.stdout)
		.ok()
		.map(|stdout| stdout.trim().to_string())
}

/// A GitHub token from `GH_TOKEN`/`GITHUB_TOKEN`, or from the GitHub CLI
fn session_token(worktree: &Worktree) -> Option<String> {
	worktree
		.shell_env()
		.into_iter()
		.find(|(key, value)| (key == "GH_TOKEN" || key == "GITHUB_TOKEN") && !value.is_empty())
		.map(|(_, value)| value)
		.or_else(|| run(worktree, "gh", &["auth", "token"]))
		.filter(|token| !token.is_empty())
}

/// The `owner/name` of a github.com remote URL (SSH or HTTPS)
fn github_slug(remote: &str) -> Option<String> {
	let path = remote
		.strip_prefix("git@github.com:")
		.or_else(|| remote.strip_prefix("ssh://git@github.com/"))
		.or_else(|| remote.strip_prefix("https://github.com/"))
		.or_else(|| remote.split_once("@github.com/").map(|(_, path)| path))?;
	let slug = path.trim_end_matches('/').trim_end_matches(".git");
	(slug.split('/').count() == 2).then(|| slug.to_string())
}

/// A `file://` URI encoded the same way as the document URIs Zed sends to the server
fn file_uri(path: &str) -> String {
	let mut uri = String::from("file://");
	for byte in path.bytes() {
		match byte {
			b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' => {
				uri.push(byte as char)
			}
			_ => uri.push_str(&format!("%{byte:02X}")),
		}
	}
	uri
}

/// The repository context the server needs to fetch secrets, variables and environments from github.com
fn repository_context(worktree: &Worktree) -> Option<serde_json::Value> {
	let root = worktree.root_path();
	let remote = run(
		worktree,
		"git",
		&["-C", &root, "remote", "get-url", "origin"],
	)?;
	let slug = github_slug(&remote)?;
	let repo: serde_json::Value =
		serde_json::from_str(&run(worktree, "gh", &["api", &format!("repos/{slug}")])?).ok()?;

	Some(serde_json::json!({
		"id": repo["id"].as_u64()?,
		"owner": repo["owner"]["login"].as_str()?,
		"name": repo["name"].as_str()?,
		"organizationOwned": repo["owner"]["type"].as_str() == Some("Organization"),
		"workspaceUri": file_uri(&root),
	}))
}

/// Recursively overrides `base` with `overrides`
fn merge(base: &mut serde_json::Value, overrides: serde_json::Value) {
	match (base, overrides) {
		(serde_json::Value::Object(base), serde_json::Value::Object(overrides)) => {
			for (key, value) in overrides {
				merge(base.entry(key).or_insert(serde_json::Value::Null), value);
			}
		}
		(base, overrides) => *base = overrides,
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
		language_server_id: &LanguageServerId,
		worktree: &Worktree,
	) -> Result<Option<serde_json::Value>> {
		let mut options = serde_json::json!({ "sessionToken": "" });

		// Without a token or a github.com remote, the server still runs without the github.com validations
		if let Some(token) = session_token(worktree) {
			options["sessionToken"] = serde_json::Value::String(token);
			if let Some(repo) = repository_context(worktree) {
				options["repos"] = serde_json::json!([repo]);
			}
		}

		if let Some(overrides) =
			LspSettings::for_worktree(language_server_id.as_ref(), worktree)?.initialization_options
		{
			merge(&mut options, overrides);
		}
		Ok(Some(options))
	}
}

register_extension!(GitHubActionsExtension);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_github_remotes() {
		for remote in [
			"git@github.com:owner/repo.git",
			"ssh://git@github.com/owner/repo.git",
			"https://github.com/owner/repo",
			"https://user:token@github.com/owner/repo.git/",
		] {
			assert_eq!(
				github_slug(remote).as_deref(),
				Some("owner/repo"),
				"{remote}"
			);
		}
		assert_eq!(github_slug("git@gitlab.com:group/project.git"), None);
	}

	#[test]
	fn encodes_workspace_uri() {
		assert_eq!(
			file_uri("/Users/me/My Projects/repo"),
			"file:///Users/me/My%20Projects/repo"
		);
	}

	#[test]
	fn user_options_override_detected_ones() {
		let mut options = serde_json::json!({ "sessionToken": "detected", "repos": [] });
		merge(
			&mut options,
			serde_json::json!({ "sessionToken": "mine", "logLevel": 1 }),
		);
		assert_eq!(
			options,
			serde_json::json!({ "sessionToken": "mine", "repos": [], "logLevel": 1 })
		);
	}
}
