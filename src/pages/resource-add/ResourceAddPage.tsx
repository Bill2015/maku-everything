import { useState } from 'react';
import { Box, Grid, Text } from '@mantine/core';
import { ResourceDisplay } from '@pages/resource-detail/components/ResourceDisplay';
import { useActiveCategoryRedux } from '@store/global';
import { AddPageFunctionSide } from './components';
import { TextTagMapperProvider } from './hooks';
import { Carousel } from '@mantine/carousel';
import { ResourceThumbnailDisplayer } from '@components/display';

import '@mantine/carousel/styles.css';
import classes from './ResourceAddPage.module.scss';

type Resource = {
    path: string;
}


const mockResource: Resource[] = [
    {
        path: 'D:\\GithubRepo\\maku everything\\dataset\\hololive\\irys-[hololive]-holoen-thumb-anime-girl-gif-69930183607745831126993018360774583112',
    },
    {
        path: 'D:\\GithubRepo\\maku-everything\\src\\pages',
    },
];

export function ResourceAddPageContent() {
    const { activeCategory } = useActiveCategoryRedux();
    const [text, setText] = useState<string>(mockResource[0].path);

    const handleSlideChange = (index: number) => {
        setText(mockResource[index].path);
    }

    return (
        <Grid classNames={{ inner: classes.innerGrid }} miw={0} mih={0}>
            <Grid.Col p={0} span={{ lg: 6, sm: 12 }} mah="100%" ta="center" display="flex">
                <Carousel
                    slideGap="lg"
                    loop
                    withIndicators
                    classNames={{ root: classes.carouselRoot, slide: classes.carouselSlide }}
                    onSlideChange={handleSlideChange}
                >
                    {
                        mockResource.map((val) => (
                            <Carousel.Slide>
                                <Text>{val.path}</Text>
                            </Carousel.Slide>
                        ))
                    }
                </Carousel>
            </Grid.Col>
            <Grid.Col span={{ lg: 6, sm: 12 }} mah="100%">
                <AddPageFunctionSide text={text} />
            </Grid.Col>
        </Grid>
    );
}

export default function ResourceAddPage() {
    return (
        <TextTagMapperProvider>
            <ResourceAddPageContent />
        </TextTagMapperProvider>
    );
}
