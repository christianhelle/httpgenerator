# HTTP File Generator Documentation Site

This directory contains the static documentation website for the HTTP File Generator project.

The canonical source layout for the product now lives under `src\rust`, `src\dotnet`, and `src\vscode`; this `docs/` folder remains a root-level documentation surface alongside the preserved repo-root build entrypoints.

The docs site is also the human-facing homepage referenced by the public Rust crates. Keep its installation guidance aligned with the root `README.md`: `cargo install httpgenerator` is the Rust ecosystem install path for published releases, `https://christianhelle.com/httpgenerator/install` and `install.ps1` provide one-line CLI installers, GitHub Releases provide the raw prebuilt CLI archives, and the editor extensions continue distributing bundled native binaries through platform-targeted `.vsix` packages. For the VS Code extension, document the runtime lookup contract as `http-file-generator.executablePath` → bundled binary → repo-root workspace `target\debug` / `target\release` → `PATH`, and avoid describing crates.io or the legacy `.NET Tool` as extension installers.

The public crates are `httpgenerator` and `httpgenerator-core`. The `httpgenerator-core` crate now also owns the `httpgenerator_core::openapi::*` API surface. Differential compatibility coverage remains internal test-only support and should not be presented as a user-installable crate.

## Files

- `index.html` - Main documentation page with complete project information
- `styles.css` - Stylesheet with light/dark mode support
- `script.js` - JavaScript for theme switching and interactive features

## Features

- 📱 **Mobile responsive design**
- 🌙 **Dark/light mode toggle** with automatic system preference detection
- 🎨 **Clean, professional typography** using Inter font family
- ⚡ **Lightweight and fast** - minimal JavaScript, optimized CSS
- 📋 **Copy-to-clipboard** functionality for code blocks
- 🖼️ **Optimized images** with proper alt text and responsive sizing
- ♿ **Accessible design** with proper semantic HTML and focus indicators

## GitHub Pages Deployment

This site is designed to be deployed via GitHub Pages from the `docs/` folder. The site uses:

- Static HTML/CSS/JS (no build process required)
- Relative paths for images and assets
- SEO-optimized meta tags
- Performance optimizations

## Development

To run locally:

```bash
cd docs
python3 -m http.server 8080
```

Then visit `http://localhost:8080`

Build and packaging entrypoints for the product remain at the repository root (`Cargo.toml`, `src/dotnet/HttpGenerator.slnx`, and `src\vscode\build.ps1`), even though implementation sources now live under `src\`.

## Theme

The site uses a modern, clean design with:
- Inter font family for readability
- Fira Code for code blocks
- CSS custom properties for theming
- Smooth transitions and animations
- Professional color scheme supporting both light and dark modes
