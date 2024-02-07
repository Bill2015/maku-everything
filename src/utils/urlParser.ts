import { YoutubePrefix } from '@declares/variables';

/**
 * get Youtube Video Hash Id \
 * For example:
 * ```typescript
 * // It will output `uWxGMzSdX0c`
 * getYoutubeVideoId('https://www.youtube.com/watch?v=uWxGMzSdX0c&list=RDuWxGMzSdX0c&start_radio=1');
 * ```
 *
 * @param urlString Youtube Video URL (including shorts)
 * @returns if is valid Youtube URL return `Hash ID` otherwise `null` */
export function getYoutubeVideoId(url: string) {
    let youtubeId: string | null = null;

    if (url.startsWith(YoutubePrefix.NormalVideo)) {
        youtubeId = url.substring(YoutubePrefix.NormalVideo.length);
    }
    else if (url.startsWith(YoutubePrefix.ShortVideo)) {
        youtubeId = url.substring(YoutubePrefix.ShortVideo.length);
    }

    if (youtubeId) {
        const paramIndex = youtubeId.indexOf('&');
        youtubeId = (paramIndex > 0) ? youtubeId.slice(0, paramIndex) : youtubeId;
        const queryIndex = youtubeId.indexOf('?');
        youtubeId = (queryIndex > 0) ? youtubeId.slice(0, queryIndex) : youtubeId;
        return youtubeId;
    }

    return null;
}

/**
 * Get Name and Extension from path
 * For example:
 * ```typescript
 * // It will output `[test, png]`
 * getYoutubeVideoId('D:/Repo/maku-everything/dataset/test.png');
 * ```
 * @param path File Path
 * @returns [`FileName`, `Extension`] */
export function getNameAndExtFromPath(path: string): [string, string] {
    const str = path.split(/[\\/]/gi);

    const getNameAndExt = (fileName: string): [string, string] => {
        const index = fileName.lastIndexOf('.');
        if (index >= 0) {
            return [fileName.substring(0, index), fileName.substring(index + 1)];
        }
        return [fileName, ''];
    };

    if (str.length <= 0) {
        return getNameAndExt(path);
    }

    const lastElement = str.pop()!;
    return getNameAndExt(lastElement);
}

/**
 * Normalize the string, prevent special character in string
 * @param str string
 * @returns pure string (including Unicode character) */
export function stringNormalize(str: string) {
    return str.replace(/[&\\/\\#,+()$~%.'":*?<>{}\s]/g, '');
}
