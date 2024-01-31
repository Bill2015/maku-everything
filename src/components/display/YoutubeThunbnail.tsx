/* eslint-disable react/jsx-props-no-spreading */
import { CiImageOff } from 'react-icons/ci';
import { Center, Image } from '@mantine/core';
import { getYoutubeVideoId } from '@utils/urlParser';

import { useMemo, useRef, useState } from 'react';
import { ResponsiveImageProps } from './ResponsiveImage';

export interface YoutubeThumbnailProps extends Omit<ResponsiveImageProps, 'src' | 'ref'> {
    url: string;

    useBackgoundImg?: boolean;
}

/**
 * Display Youtube Video Thumbnail \
 * Normally, It will display **Max Resolution** Image, if not it will get **second high resolution** image to display \
 *
 * #### Max Resolution
 * https://img.youtube.com/vi/${videoId}/maxresdefault.jpg
 *
 * #### Default
 * https://img.youtube.com/vi/${videoId}/0.jpg */
export function YoutubeThumbnail(props: YoutubeThumbnailProps) {
    const { url, alt, useBackgoundImg = false, ...imgProps } = props;

    const [isLoaded, setIsLoaded] = useState<boolean>(false);
    const ref = useRef<HTMLImageElement | null>(null);

    function createURL(videoId: string, type: 'default' | 'max-resoultion') {
        return `https://img.youtube.com/vi/${videoId}/${(type === 'default') ? '0' : 'maxresdefault'}.jpg`;
    }

    const imageURL = useMemo(() => {
        const videoId = getYoutubeVideoId(url);
        if (!videoId) {
            return '';
        }

        if (!ref || !ref.current || !isLoaded) {
            return createURL(videoId, 'max-resoultion');
        }
        const w = ref.current!.naturalWidth;
        const h = ref.current!.naturalHeight;

        // Because the youtube image will return 120x90 default image if no resource found.
        // Therefore we can use size to evaluate dose image have max resoultion version or not.
        // NOTE: Because CORS, so we can't use request to determine.
        if (w <= 130 && h <= 100) {
            return createURL(videoId, 'default');
        }

        return createURL(videoId, 'max-resoultion');
    }, [url, isLoaded]);

    const sourceProps = useBackgoundImg ? { style: { backgroundImage: `url(${imageURL})` } } : { src: imageURL, alt: alt };

    if (!imageURL) {
        return (
            <Center>
                <CiImageOff style={{ width: '50%', height: '50%' }} />
            </Center>
        );
    }

    return (
        <Image
            ref={ref}
            onLoad={() => setIsLoaded(true)}
            src={imageURL}
            {...imgProps}
            {...sourceProps}
        />
    );
}
