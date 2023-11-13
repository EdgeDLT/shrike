# GUI

The GUI is a user-friendly web interface built with SolidJS (SolidStart) and PicoCSS. It provides a simple way for users to interact with the data provided by Shrike.

## Features

- User-friendly interface for interacting with Shrike data.
- Built with SolidJS and PicoCSS for a lightweight and efficient user experience.
- Suitable for both regular and power users.

## Getting Started

### Prerequisites

- Node.js and npm installed on your machine. You can download them from [here](https://nodejs.org/en/download/).
- A running instance of the Shrike API.

### Quickstart

1. Clone or download the GUI folder.
2. Update the path in `/constants/index.js` to point to your running Shrike API instance.
3. Install the dependencies by running `npm install`.
4. Serve the GUI locally using `npm run dev`. You can then open it in your browser at `http://127.0.0.1:5173/`.

## Contributing

Contributions to the GUI are welcomed. If you have suggestions for improvements or additional features, feel free to open an issue or submit a pull request.

-----

## SolidStart

Everything you need to build a Solid project, powered by [`solid-start`](https://start.solidjs.com);

### Creating a project

```bash
# create a new project in the current directory
npm init solid@latest

# create a new project in my-app
npm init solid@latest my-app
```

### Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

### Building

Solid apps are built with _adapters_, which optimise your project for deployment to different environments.

By default, `npm run build` will generate a Node app that you can run with `npm start`. To use a different adapter, add it to the `devDependencies` in `package.json` and specify in your `vite.config.js`.
