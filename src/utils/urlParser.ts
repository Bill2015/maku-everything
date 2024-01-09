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
    }

    return youtubeId;
}
