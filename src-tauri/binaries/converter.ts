/**
 * Generation of binary file suffixes depends on the operating system
 * In project root path use `npm run prepare` to generate suffix
 * See Also: https://tauri.app/v1/guides/building/sidecar/ */
import * as fs from 'fs';
import { execa } from 'execa';


const includeExt = ['.exe', ''];
async function main() {
    const rustInfo = (await execa('rustc', ['-vV'])).stdout;
    const targetTriple = /host: (\S+)/g.exec(rustInfo)![1];
    if (!targetTriple) {
        console.error('Failed to determine platform target triple')
    }
    fs.readdir('./src-tauri/binaries/', (err, filenames) => {
        for(const fileName of filenames) {
            const index = fileName.lastIndexOf('.');
            const ext = (index > 0) ? fileName.substring(index + 1) : '';
            if (includeExt.includes(ext) && !fileName.includes(targetTriple)) {
                fs.renameSync(
                    `src-tauri/binaries/${fileName}${ext}`,
                    `src-tauri/binaries/${fileName}-${targetTriple}${ext}`
                )
            }
        }
    });
}

main().catch((e) => {
    throw e
})
