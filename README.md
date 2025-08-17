# zed-github-actions
[![Zed Extension][zed-extension-badge]][zed-extension-url]
[![License][license-badge]][license-url]

[zed-extension-badge]: https://img.shields.io/badge/Zed%20Extension-%230951CF?style=flat-square&logo=zedindustries&logoColor=white&labelColor=black
[zed-extension-url]: https://zed.dev/extensions/github-actions
[license-badge]: https://img.shields.io/badge/License-Apache%202.0-blue?style=flat-square&labelColor=black&color=blue
[license-url]: #license

GitHub Actions LSP support for Zed. As this repository uses code based on some official Zed extensions (e.g [Svelte](https://github.com/zed-extensions/svelte) and [Astro](https://github.com/zed-extensions/astro)), this repository is under the same license as those.
To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

- Tree-sitter: [zed-industries/tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml)
- Language Server: [actions/languageservices](https://github.com/actions/languageservices)
- Language Server (binary): [lttb/gh-actions-language-server](https://github.com/lttb/gh-actions-language-server)

> [!NOTE]
> The LSP provided by the official NPM package [`@actions/languageserver`](https://www.npmjs.com/package/@actions/languageserver) does not include a binary (see [issue #56](https://github.com/actions/languageservices/issues/56)), so this repository uses a third-party NPM package to install it via [`gh-actions-language-server`](https://www.npmjs.com/package/gh-actions-language-server).

## Configuring
### Filetype settings
This extension by default does not have any file associations built-in, as Zed doesn't support glob patterns at the extension-level to recognize a language within a specific directory. Instead, you can edit your Zed settings file (`settings.json`) with:

```jsonc
{
	// ...
	"file_types": {
		"GitHub Actions": [
			".github/workflows/*.yml",
			".github/workflows/*.yaml"
		]
	}
}
```

This extension avoids conflicting with the built-in YAML support for Zed by following how other Zed extensions for specific YAML files resolve this issue, including the [Ansible](https://github.com/kartikvashistha/zed-ansible) extension and [Docker Compose](https://github.com/eth0net/zed-docker-compose) extension.

### LSP settings
You can configure the LSP settings in Zed with:

```jsonc
{
	// ...
	"lsp": {
		"gh-actions-language-server": {
			"initialization_options": {
				// ...
			}
		}
	}
}
```

#### Default settings
The default settings (`initialization_options`) set by the extension is shown below:
```jsonc
{
	"sessionToken": ""
}
```

A session token is a GitHub PAT (Personal Access Token). This is not required, but when specified allows accessing more information from github.com.
- [Classic PATs](https://github.com/settings/tokens/new) will need access to `repo` and `workflow` scopes.
- [Fine-grained PATs](https://github.com/settings/personal-access-tokens/new), which can either be given access to:
  - "Public repositories"
  - "All repositories"/"Only select repositories" with repository permissions to `Workflows`

## License
Licensed under Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>).

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
