# Use Binary files
## Description
This folder is stored the `Binary Files` Which is needed by the App. \
See Also: [Embedding External Binaries](https://tauri.app/v1/guides/building/sidecar/)

<b>
Note: <font color="red">those binary files can't not including in the Git</font>
<br />
(because I not sure those binary files size will cause git system heavy loadout if it's too large)
</b>

## 1. Require Binaries 
- [SurrealDB](https://docs.surrealdb.com/docs/installation/windows)
    > **NOTE:** need to rename to <font color="aqua">**surreal**</font>

## 2. Generate the suffix
When you add binary into this folder. \
To the `root of project` and run this command like below:
```bash
npm run prepare
```
It will generate the suffix depends on the operating system.
