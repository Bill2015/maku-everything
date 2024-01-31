/* eslint-disable react/jsx-props-no-spreading */
import * as lodash from 'lodash';
import { Image, ImageProps, Loader } from '@mantine/core';
import { useState } from 'react';

import classes from './ResponsiveImage.module.scss';

export interface ResponsiveImageProps extends ImageProps {
    alt: string;

    useBackgoundImg?: boolean;
}

export function ResponsiveImage(props: ResponsiveImageProps) {
    const { alt, src, useBackgoundImg = false, ...imgProps } = props;
    const [isLoaded, setLoaded] = useState<boolean>(true);

    const sourceProps = useBackgoundImg ? { style: { backgroundImage: `url("${src}")` } } : { src: src, alt: alt };

    return (
        <>
            { !isLoaded && (
                <Loader
                    w="100%"
                    display="flex"
                    size="lg"
                    h={lodash.random(100, 300)}
                    style={{ alignItems: 'center', justifyContent: 'center' }}
                />
            ) }
            <Image
                onLoad={() => setLoaded(true)}
                hidden={!isLoaded}
                className={classes.responsiveImg}
                {...imgProps}
                {...sourceProps}
            />
        </>
    );
}
