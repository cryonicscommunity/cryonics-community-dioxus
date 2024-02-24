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

## Hot Reload

- cd cryonics-community-dioxus/
- npm install
  - Installs the utility http-server to serve the HTML
  - Installs the utility pretter to format the HTML
  - Installs the utility rimraf to remove distribution directory dist/
- npm start
- Open your browser to http://localhost:8080/
- Make changes to the HTML in src/lib.rs or the CSS in public/stylesheet.css
- Note that the changes are updated in your browser as soon as you save

## Test Static Prerendering with Hydration

- npm test
  - Deletes the distribution directory dist/ to start clean
  - Makes the index.html page with the hydration code
  - Inserts the prerendered HTML
  - Formats the HTML using the prettier utility
  - Launches http-server to serve the HTML
  - Opens your browser to the home page

## Other Commands

- npm run clean
  - Deletes the distribution directory dist/ to start clean
- npm run dist
  - Same as npm test
  - Except that it does not start http-server and open the browser
- npm run format
  - Runs the utility prettier
- npm run hydrate
  - Makes the index.html page with the hydration code
- npm run prerender
  - Inserts the prerendered HTML
- npm run make
  - Makes the index.html page with the hydration code
  - Inserts the prerendered HTML
  - Runs the utility prettier
  - But does not start by deleting dist/
- npm run serve
  - Starts the http-server
  - Opens the browser

## Links

- CroftSoft Rust-Dioxus Project Setup Tutorial
  - https://www.croftsoft.com/library/tutorials/rust-dioxus-project-setup/

## History

- Initial release: 2024-02-23
