/* eslint-disable react/jsx-props-no-spreading */
import { LocalImageThumbnail, LocalUnsupportThumbnail } from './file';
import { LocalThumbnailProps } from './file/types';

const LOCAL_MEDIA_TYPE = new Map<string, typeof LocalImageThumbnail>();

LOCAL_MEDIA_TYPE.set('image/jpeg', LocalImageThumbnail);
LOCAL_MEDIA_TYPE.set('image/png', LocalImageThumbnail);
LOCAL_MEDIA_TYPE.set('image/gif', LocalImageThumbnail);

interface LocalFileThumbnailDisplayerProps extends LocalThumbnailProps {
    mediaType: string;
}

export function LocalFileThumbnailDisplayer(props: LocalFileThumbnailDisplayerProps) {
    const { filePath, mediaType, ...imgProps } = props;

    if (LOCAL_MEDIA_TYPE.has(mediaType) === false) {
        return <LocalUnsupportThumbnail mediaType={mediaType} {...imgProps} />;
    }

    const Thumbnail = LOCAL_MEDIA_TYPE.get(mediaType)!;

    return <Thumbnail filePath={filePath} {...imgProps} />;
}
