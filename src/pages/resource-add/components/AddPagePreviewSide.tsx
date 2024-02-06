import { useEffect, useRef, useState } from 'react';
import { Carousel, Embla } from '@mantine/carousel';
import { ResourceThumbnailDisplayer } from '@components/display';

import '@mantine/carousel/styles.css';
import classes from './AddPagePreviewSide.module.scss';

export interface AddPagePreviewSideProps {
    data: string[];

    onSlideChange: (index: number) => void;
}

export function AddPagePreviewSide(props: AddPagePreviewSideProps) {
    const { data, onSlideChange } = props;
    const [embla, setEmbla] = useState<Embla | null>(null);

    const sizeObserver = useRef<ResizeObserver>(new ResizeObserver(() => {}));

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

    return (
        <Carousel
            slideGap="lg"
            loop
            withIndicators
            classNames={{ root: classes.carouselRoot, slide: classes.carouselSlide }}
            onSlideChange={onSlideChange}
            getEmblaApi={setEmbla}
        >
            {
                data.map((val) => (
                    <Carousel.Slide>
                        <ResourceThumbnailDisplayer url={val} alt={val} />
                    </Carousel.Slide>
                ))
            }
        </Carousel>
    );
}
