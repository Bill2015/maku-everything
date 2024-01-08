import { useMemo } from 'react';
import { CiImageOff } from 'react-icons/ci';

import { Center } from '@mantine/core';
import { ResponsiveImage, ResponsiveImageProps } from './ResponsiveImage';

const YOUTUBE_PREFIX = 'https://www.youtube.com/watch?v=';
const YOUTUBE_SHORT_PREFIX = 'https://www.youtube.com/shorts/';

export interface YoutubeThumbnailProps extends Omit<ResponsiveImageProps, 'src'> {
    url: string;
}

export function YoutubeThumbnail(props: YoutubeThumbnailProps) {
    const { url } = props;

    const videoHash = useMemo(() => {
        if (url.startsWith(YOUTUBE_PREFIX)) {
            return url.substring(YOUTUBE_PREFIX.length);
        }
        if (url.startsWith(YOUTUBE_SHORT_PREFIX)) {
            return url.substring(YOUTUBE_SHORT_PREFIX.length);
        }
        return '';
    }, [url]);

    if (!videoHash) {
        return (
            <Center>
                <CiImageOff style={{ width: '50%', height: '50%' }} />
            </Center>
        );
    }
    return (
        <ResponsiveImage
            src={`https://img.youtube.com/vi/${videoHash}/maxresdefault.jpg`}
        />
    );
}
