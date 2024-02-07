import { useEffect, useRef, useState } from 'react';
import { Carousel, Embla } from '@mantine/carousel';
import { ActionIcon, Kbd, Stack, Text } from '@mantine/core';
import { FaRegTrashCan } from 'react-icons/fa6';
import { ResourceThumbnailDisplayer } from '@components/display';

import '@mantine/carousel/styles.css';
import classes from './AddPagePreviewSide.module.scss';
import { useAddResourceContext } from '../stores';

const carouselClasses = {
    root:      classes.carouselRoot,
    slide:     classes.carouselSlide,
    viewport:  classes.carouselViewPort,
    container: classes.carouselContainer,
};

export function AddPagePreviewSide() {
    const [embla, setEmbla] = useState<Embla | null>(null);
    const { resources, activeResource, deleteResource, setActiveResource } = useAddResourceContext();

    const sizeObserver = useRef<ResizeObserver>(new ResizeObserver(() => {}));

    // when data changed, scroll to the end
    useEffect(() => {
        if (embla && activeResource) {
            embla.scrollTo(activeResource.index, false);
        }
    }, [activeResource, embla]);

    // if Carousel size re-init it to calculated right width
    useEffect(() => {
        if (!embla) {
            return;
        }
        if (sizeObserver) {
            sizeObserver.current?.disconnect();
        }
        sizeObserver.current = new ResizeObserver(() => {
            embla.reInit();
        });
        sizeObserver.current.observe(embla.rootNode());
        return () => sizeObserver.current.disconnect();
    }, [embla]);

    // binding key event (For UX)
    useEffect(() => {
        const controller = new AbortController();
        window.addEventListener('keydown', (e) => {
            switch (e.key) {
            case 'ArrowRight':
                embla?.scrollNext();
                break;
            case 'ArrowLeft':
                embla?.scrollPrev();
                break;
            default:
                break;
            }
        }, { signal: controller.signal });

        return () => controller.abort();
    }, [embla]);

    if (resources.length <= 0) {
        return (
            <Stack justify="center">
                <Text fz="xl">
                    Drag file into the App
                </Text>
                <Text fz="xl">
                    Or Use
                    {' '}
                    <Kbd>Ctrl</Kbd>
                    {' '}
                    +
                    {' '}
                    <Kbd>V</Kbd>
                    {' '}
                    to paste the URL in the App
                </Text>
            </Stack>
        );
    }

    return (
        <>
            <ActionIcon
                variant="outline"
                className={classes.deleteIcon}
                onClick={() => {
                    if (embla) {
                        deleteResource(embla.selectedScrollSnap());
                    }
                }}
            >
                <FaRegTrashCan />
            </ActionIcon>
            <Carousel
                slideGap="lg"
                loop
                withIndicators
                classNames={carouselClasses}
                getEmblaApi={setEmbla}
                onSlideChange={setActiveResource}
            >
                {
                    resources.map((val) => (
                        <Carousel.Slide key={`${val.file_path}${val.url_path}`}>
                            <ResourceThumbnailDisplayer filePath={val.file_path} url={val.url_path} alt="" />
                        </Carousel.Slide>
                    ))
                }
            </Carousel>
        </>
    );
}
