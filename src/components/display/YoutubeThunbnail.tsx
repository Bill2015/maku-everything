import { useMemo } from 'react';
import { CiImageOff } from 'react-icons/ci';
import { Center } from '@mantine/core';
import { YoutubePrefix } from '@declares/variables';

import { ResponsiveImage, ResponsiveImageProps } from './ResponsiveImage';

export interface YoutubeThumbnailProps extends Omit<ResponsiveImageProps, 'src'> {
    url: string;
}

export function YoutubeThumbnail(props: YoutubeThumbnailProps) {
    const { url } = props;

    const videoHash = useMemo(() => {
        if (url.startsWith(YoutubePrefix.NormalVideo)) {
            return url.substring(YoutubePrefix.NormalVideo.length);
        }
        if (url.startsWith(YoutubePrefix.ShortVideo)) {
            return url.substring(YoutubePrefix.ShortVideo.length);
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
