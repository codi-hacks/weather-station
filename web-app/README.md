# Progressive web app

- [Overview](#overview)
- [Getting started](#getting-started)
- [Deploy to production](#deploy-to-production)
- [Testing](#testing)
- [Customize the configuration](#customize-the-configuration)

## Overview

This is the user's web app for fetching weather station data from the API server and displaying it in the user's browser on their computer or mobile device.
Specifically, it is a PWA (progressive web app) which is a website that can be installed as an icon on the user's home screen and accessed the same way they would a native app.

For an example on how to install PWA's such as this, see [this demo](https://httpd.apache.org/).

For more information on PWA's check out this introduction and guide on [web.dev](https://web.dev/progressive-web-apps/).

## Getting started

1. Install an up-to-date copy of [nodejs](https://nodejs.org) (current or LTS both work)
2. Install node modules:

```sh
npm install
```
3. Copy "config.json.example" to "config.json"
4. Update the API URL as appropriate.
5. Start the development web server to view the website locally

```sh
npm start
```

At this point you are ready to begin development.


## Deploy to production

This command compiles and minifies the source code into a static build for production:

```sh
npm run build
```

This will output a `dist/` folder, the contents of which can be served in the public folder of production-ready HTTP server such as [nginx](https://nginx.org/en/) or [Apache HTTP server](https://httpd.apache.org/).

## Testing

The only testing in place is a linter, which can be ran with the following command:

```
yarn lint
```

The linter also runs during development so this command isn't something you'll need to run very often.


## Customize the configuration

See the official [configuration reference](https://cli.vuejs.org/config/).
