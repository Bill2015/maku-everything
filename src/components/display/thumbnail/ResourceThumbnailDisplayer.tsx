/* eslint-disable react/jsx-props-no-spreading */
import { ResponsiveImageProps } from '@components/display';

import { YoutubeThumbnail } from './url';
import { LocalFileThumbnailDisplayer } from './LocalFileThumbnailDisplayer';

export interface ResourceThumbnailDisplayerProps extends ResponsiveImageProps {
    url?: string | undefined | null;

    filePath?: string | undefined | null;

    mediaType?: string;
}

export function ResourceThumbnailDisplayer(props: ResourceThumbnailDisplayerProps) {
    const { url, filePath, mediaType, ...imgProps } = props;

    return (
        url
            ? <YoutubeThumbnail url={url!} {...imgProps} />
            : (
                <LocalFileThumbnailDisplayer
                    mediaType={mediaType!}
                    filePath={filePath!}
                    {...imgProps}
                />
            )
    );
}
