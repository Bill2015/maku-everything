import { useState } from 'react';
import { Grid } from '@mantine/core';
import { useActiveCategoryRedux } from '@store/global';
import { AddPageFunctionSide, AddPagePreviewSide } from './components';
import { TextTagMapperProvider } from './hooks';

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
    {
        path: 'https://www.youtube.com/watch?v=WwAhs9SqBfM',
    },
    {
        path: 'https://www.youtube.com/watch?v=1otZy9RgJFQ&t=4479s',
    },
    {
        path: 'https://www.youtube.com/watch?v=eETFCBmC_JY',
    }
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
                <AddPagePreviewSide data={mockResource.map((val) => val.path)} onSlideChange={handleSlideChange} />
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
