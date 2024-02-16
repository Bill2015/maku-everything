<h1 align="center">Maku Everything</h1>
Maku-Everything is a tagging system similar to Danbooru or Gelbooru that allows you to manage any kind of files and documents with a powerful search and tagging system that makes it easier to find the file, and the tags don't affect the original file name. In addition, the program is completely independent and does not require the installation of any other software.

## 🔧 Features
- 👓 Modern and intuitive interface
- ⚡ Lightweight and fast
- 🔍 Powerful resource search system
- 🏷️ Multi-Level Label Management
- 🌐 Multi-language supported
- 🔗 Can also add web links to the software
- 📑 **Not** modify any local files (FileName, NTFS..)

## 🖥️ Demonstrations


## 📥 Desktop App Download
> [!WARNING]
> This App still work in progress, so there may be some bugs or unfinished functionality

## 🗺️ Road Map
- Improved error handling & message notification
- More URL thumbnail supported (Twitter, Bluesky...)
- Smart resources importer
  > It will using web crawling to fetch some important text and mapping with tags \
  > For example: `youtube channel name`, `twitter username`, `hashtags`, etc
- Advance tag
  > can add some text, value when tagging in a resourcs
- More powerful reosurces search system
- UI redesign
- More language supported


## ⚒️ Development
This project is built using [Tauri](https://tauri.app/) and [React](https://react.dev/).

### Prerequisites
0. Fork or Clone this repository 
1. Install the [NodeJS](https://nodejs.org/zh-tw/download)
2. Follow the Tauri [prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/) installation instructions
3. Install the [SurealDB](https://github.com/surrealdb/surrealdb/releases/tag/v1.0.0-beta.9%2B20230402)
    > This database is where our application stores data
    > Currently, stable version is **<font color="red">Surealdb-1.2.0</font>**
4. Put surrealDB executable into [binaries folder](https://github.com/Bill2015/maku-everything/tree/master/src-tauri/binaries)

### Install The App
And the path to this Project
Install the necessary dependencies
```sh
npm install
```

### Live Development
To start the App
Run this command to start dev mode
```sh
npm run tauri dev
```

### Building
To build a redistributable, production mode package
```sh
npm run tauri buld
```
