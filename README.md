<h1 align="center">Maku Everything</h1>

Maku-Everything is a tagging system similar to [Danbooru](https://danbooru.donmai.us/) or [Gelbooru](https://gelbooru.com/) that allows you to manage any kind of files and documents with a powerful search and tagging system that makes it easier to find the file, and the tags don't affect the original file name. \
In addition, the program is completely independent and does not require the installation of any other software.

> [!IMPORTANT]  
> **This App still work in progress, I'm not ready to release a version. :(** \
> If you are interested in this project, you can give me a **Github Star⭐** or [suggest more](https://github.com/Bill2015/maku-everything/discussions/19) features in this project.

## 🔧 Features
- 👓 Modern and intuitive interface
- ⚡ Lightweight and fast
- 🔍 Powerful resource search system
- 📌 Powrful tag system
- 🏷️ Multi-Level Label Management
- 🌐 Multi-language supported
- 🔗 Can also add web links to the software
- 📑 **Not** modify any local files (FileName, NTFS..)

## 🖥️ Demonstrations
> [!NOTE] 
> Some features are just temporary. it may change in the future.

### Create Category
Create a category, the category is the root of `resources`, `tag topics`, `tags` and contains a root directory to add resources, resources with different root directories cannot be added.

https://github.com/Bill2015/maku-everything/assets/63895869/4590831c-eda5-4383-bca1-7b05d2259a06

---
### Tagging
<img align="right" src="https://github.com/Bill2015/maku-everything/assets/63895869/ac8f0a15-284b-40b2-8cff-0dafd76252db">

The tag is based on danbooru, but not only that, but we also provide more `tag types`, including `text`, `number`, `date`, etc. \
That means you don't need to duplicate or create some similar tags, like `rating`, `information`.

You can see this problem on the right side pictures. this image is from [Danbooru](https://danbooru.donmai.us/) tags, they added many similar tags to present some certain value. But you don't need to do the same thing in our App. just create a `Number Tag` or `Date Tag` to solve this problem

https://github.com/Bill2015/maku-everything/assets/63895869/08d2e11d-022e-4998-8b74-b7d6db2f4047

---
### Adding Resoures
Adding resources from locally, in this example, you can see there is a feature called `text map`, this feature can attach the tag automatically from the file name. \
In the future, we plan this feature also to get the text from the web URL, which means web crawling is necessary

https://github.com/Bill2015/maku-everything/assets/63895869/b499d3b6-b2a3-477a-aa23-9792bfd85a38

You not only have local files but also you can paste the URL to create the Resources, and the thumbnail will generated. But now only the `Youtube` link can generate a thumbnail. It will support more URLs generated in the future.

https://github.com/Bill2015/maku-everything/assets/63895869/431f908a-f127-4d63-bf63-f772f61478c6

---
### Search Resoures
This feature is another powerful feature, not only just  `exclude` and `include`, also has the autocomplete, grouping, and functional tags.

https://github.com/Bill2015/maku-everything/assets/63895869/9f2df158-5e21-4cd2-ba44-59756aeeeb65

the functional tags are pre-defined by our App. you can search resources by some resource data. for example: the number of tags that the resource has, create date, update date, etc...

https://github.com/Bill2015/maku-everything/assets/63895869/075a8221-4e53-4b6a-b8b0-82adf467c49b

---
### Export & Import
If you want to change the category root path. you can export the whole category and import it in another path.

https://github.com/Bill2015/maku-everything/assets/63895869/3ecb2bec-2a72-44f1-9e0f-eb2c13d04627


## 📥 Desktop App Download
Coming Soon!

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
    > Currently, stable version is **<font color="red">Surealdb-1.1.1</font>**
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
