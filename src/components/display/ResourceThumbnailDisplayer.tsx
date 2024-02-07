/* eslint-disable react/jsx-props-no-spreading */
import { convertFileSrc } from '@tauri-apps/api/tauri';

import { YoutubeThumbnail } from './YoutubeThunbnail';
import { ResponsiveImage, ResponsiveImageProps } from './ResponsiveImage';

interface ResourceThumbnailDisplayerProps extends ResponsiveImageProps {
    url?: string | undefined | null;

    filePath?: string | undefined | null;
}

export function ResourceThumbnailDisplayer(props: ResourceThumbnailDisplayerProps) {
    const { url, filePath, ...imgProps } = props;

    return (
        url
            ? <YoutubeThumbnail url={url!} {...imgProps} />
            : <ResponsiveImage src={convertFileSrc(filePath!)} {...imgProps} />
    );
}
