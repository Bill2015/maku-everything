import { CiImageOff } from 'react-icons/ci';
import { Center } from '@mantine/core';
import { getYoutubeVideoId } from '@utils/urlParser';

import { ResponsiveImage, ResponsiveImageProps } from './ResponsiveImage';

export interface YoutubeThumbnailProps extends Omit<ResponsiveImageProps, 'src'> {
    url: string;
}

export function YoutubeThumbnail(props: YoutubeThumbnailProps) {
    const { url } = props;

    const videoId = getYoutubeVideoId(url);

    if (!videoId) {
        return (
            <Center>
                <CiImageOff style={{ width: '50%', height: '50%' }} />
            </Center>
        );
    }
    return (
        <ResponsiveImage
            src={`https://img.youtube.com/vi/${videoId}/maxresdefault.jpg`}
        />
    );
}
