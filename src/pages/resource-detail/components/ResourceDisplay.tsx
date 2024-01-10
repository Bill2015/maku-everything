import { Carousel } from '@mantine/carousel';
import { Center, Image } from '@mantine/core';
import { useViewportSize } from '@mantine/hooks';
import { WebEmbedDisplayer } from '@components/webembed';

import '@mantine/carousel/styles.css';
import classes from './ResourceDisplay.module.scss';

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
    // because I can't let image size within the parent element
    // Therefore, I use viewport to change it.
    const { height } = useViewportSize();

    if (havePath && haveUrl) {
        return (
            <Carousel
                slideGap="lg"
                loop
                withIndicators
                classNames={{ root: classes.carouselRoot, slide: classes.carouselSlide }}
            >
                <Carousel.Slide>
                    <Image
                        height={height - 130}
                        className={classes.image}
                        alt={name}
                        src={filePath}
                    />
                </Carousel.Slide>
                <Carousel.Slide>
                    <WebEmbedDisplayer name={name} url={url!} host={host!} />
                </Carousel.Slide>
            </Carousel>
        );
    }

    if (haveUrl) {
        return (
            <Center p="md" w="100%">
                <WebEmbedDisplayer name={name} url={url!} host={host!} />
            </Center>
        );
    }

    if (havePath) {
        return (
            <Center p="md" h="100%">
                <Image
                    height={height - 120}
                    className={classes.image}
                    alt={name}
                    src={filePath}
                />
            </Center>
        );
    }
}
