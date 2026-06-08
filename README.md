# Cryonics Community

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/cryonicscommunity/cryonics-community-dioxus/blob/main/LICENSE.txt

- Cryonics Community website implementation using Dioxus
- Makes a Content Delivery Network (CDN)-compatible static HTML distribution
  - Includes static prerendering with client-side hydration

## Utilities Installation

- Install the Rust command line utility "cargo"
  - cargo is installed when you install Rust
  - https://www.rust-lang.org/
- Install the Dioxus Command Line Interface (CLI) "dx"
  - cargo install dioxus-cli
  - https://github.com/DioxusLabs/dioxus/tree/master/packages/cli
- Install npm
  - npm installs utilities such as prettier
  - npm scripts run the dx and cargo commands
  - npm can be installed by installing node.js
  - https://nodejs.org/

## Quick Start

- npm install
- npm test

## Hot Reload

- cd cryonics-community-dioxus/
- npm install
  - Installs the utility http-server to serve the HTML
  - Installs the utility prettier to format the HTML
  - Installs the utility shx to move files and remove directories
- npm start
  - Used during development
  - Builds, watches, and serves with hot reloading
  - Automatically opens a browser window
- Make changes to the HTML in src/component/home.rs
  - Or the CSS in public/stylesheet.css
- Note that the changes are updated in your browser as soon as you save

## Test Static Prerendering with Client-side Hydration

- npm test
  - Deletes the build and distribution directories to start clean
  - Makes the index.html page with the hydration code
  - Launches http-server to serve the HTML
  - Opens your browser to the home page

## Additional Run Script Commands

- npm run clean
  - Deletes the build and distribution directories to start clean
- npm run copy
  - Copies from the build to the distribution directory dist/
- npm run dist
  - Runs the clean, build, and copy scripts
  - Used to generate an SSG distribution in the dist/ directory
  - The dist/ files can be hosted on a Content Delivery Network (CDN)
- npm run format
  - Runs the "prettier" utility to format the generated files in dist/
  - Useful for analyzing or debugging the generated files
- npm run serve
  - Starts the http-server in dist/
  - Opens the browser
- npm start
  - Described in a previous section
- npm test
  - Described in a previous section

## Links

- CroftSoft Rust-Dioxus Project Setup Tutorial
  - https://www.croftsoft.com/library/tutorials/rust-dioxus-project-setup/

## History

- 2024-02-23: Initial release
- 2026-06-08: Updated from Dioxus v0.4 to v0.7
