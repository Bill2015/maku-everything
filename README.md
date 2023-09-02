# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


## How to Build
### Prerequisites
0. Fork or Clone this repository 
1. Install the [NodeJS](https://nodejs.org/zh-tw/download)
2. Follow the Tauri [prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/) installation instructions
3. Install the [SurealDB](https://github.com/surrealdb/surrealdb/releases/tag/v1.0.0-beta.9%2B20230402)
    > This database is where our application stores data
    > Currently, stable version is **<font color="red">Surealdb-Beta9</font>**

### Start The App
Path to your surrealdb install location
Then run this command from below
```sh
surreal start memory -A --auth --user root --pass root
```

And the path to this Project
Install the necessary dependencies
```sh
npm install
```

To start the App
Run this command to start dev mode
```sh
npx tauri dev
```

### Needed Tool & Extenstion
#### VScode Extension
- ESLint
    > Extension ID `dbaeumer.vscode-eslint`
- GitGraph
    > Extension ID `mhutchie.git-graph`

### Recommand Tool
- [Surrealist](https://github.com/StarlaneStudios/Surrealist/releases/tag/v1.8.0)
    > database data visualize also can easy run SurrealQL command for simple test
