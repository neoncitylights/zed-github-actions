# zed-github-actions
![Zed Extension][zed-extension-badge]
[![License][license-badge]][license-url]

[zed-extension-badge]: https://img.shields.io/badge/Zed%20Extension-%230951CF?style=flat-square&logo=zedindustries&logoColor=white&labelColor=black
[zed-extension-url]: https://zed.dev/extensions/github-actions
[license-badge]: https://img.shields.io/badge/License-Apache%202.0-blue?style=flat-square&labelColor=black&color=blue
[license-url]: #license

GitHub Actions LSP support for Zed. As this repository uses code based on some official Zed extensions (e.g [Svelte](https://github.com/zed-extensions/svelte) and [Astro](https://github.com/zed-extensions/astro)), this repository is under the same license as those.
To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

- Language Server: [actions/languageservices](https://github.com/actions/languageservices)
- Language Server (binary): [lttb/gh-actions-language-server](https://github.com/lttb/gh-actions-language-server)

## Notes
- The LSP provided by the official NPM package [`@actions/languageserver`](https://www.npmjs.com/package/@actions/languageserver) does not include a binary (see [issue #56](https://github.com/actions/languageservices/issues/56)), so this repository uses a third-party NPM package to install it via [`gh-actions-language-server`](https://www.npmjs.com/package/gh-actions-language-server).
- This extension is currently "experimental" since it technically works, but semi-conflicts with Zed's [built-in YAML support](https://zed.dev/docs/languages/yaml) via the [redhat-developer/yaml-language-server](https://github.com/redhat-developer/yaml-language-server) LSP.

## License
Licensed under Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>).

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
