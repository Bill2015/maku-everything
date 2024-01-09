import { Carousel } from '@mantine/carousel';
import { Center, Image } from '@mantine/core';
import { CiVideoOff } from 'react-icons/ci';

import '@mantine/carousel/styles.css';
import classes from './ResourceDisplay.module.scss';

const YOUTUBE_PREFIX = 'https://www.youtube.com/watch?v=';
const YOUTUBE_SHORT_PREFIX = 'https://www.youtube.com/shorts/';

interface UrlDisplayerProps {
    name: string;

    url: string;

    host: string;
}

function UrlDisplayer(props: UrlDisplayerProps) {
    const { name, host, url } = props;

    return (
        <>
            {
                (() => {
                    let youtubeId = '';
                    if (host === 'www.youtube.com') {
                        if (url.startsWith(YOUTUBE_PREFIX)) {
                            youtubeId = url.substring(YOUTUBE_PREFIX.length);
                        }
                        if (url.startsWith(YOUTUBE_SHORT_PREFIX)) {
                            youtubeId = url.substring(YOUTUBE_SHORT_PREFIX.length);
                        }
                        return (
                            <iframe
                                width="560"
                                height="315"
                                src={`https://www.youtube.com/embed/${youtubeId}`}
                                title={name}
                                frameBorder="0"
                                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                                allowFullScreen
                            />
                        );
                    }
                    if (!youtubeId) {
                        return (
                            <Center w="100%" h="100%">
                                <CiVideoOff style={{ width: '100%', height: '100%' }} />
                            </Center>
                        );
                    }
                })()
            }
        </>
    );
}

export interface ResourceDisplayProps {
    name: string;

    havePath?: boolean;

    haveUrl?: boolean;

    filePath?: string;

    url?: string;

    host?: string;
}

export function ResourceDisplay(props: ResourceDisplayProps) {
    const { name, filePath, havePath, url, haveUrl, host } = props;

    if (havePath && haveUrl) {
        return (
            <Carousel withIndicators classNames={{ root: classes.carouselRoot, viewport: classes.carouselViewport }}>
                <Carousel.Slide>
                    <Image
                        alt={name}
                        src={filePath}
                    />
                </Carousel.Slide>
                <Carousel.Slide>
                    <UrlDisplayer name={name} url={url!} host={host!} />
                </Carousel.Slide>
            </Carousel>
        );
    }

    if (haveUrl) {
        return (
            <Center p="md">
                <UrlDisplayer name={name} url={url!} host={host!} />
            </Center>
        );
    }

    if (havePath) {
        return (
            <Center p="md">
                <Image
                    height="auto"
                    alt={name}
                    src={filePath}
                />
            </Center>
        );
    }
}
