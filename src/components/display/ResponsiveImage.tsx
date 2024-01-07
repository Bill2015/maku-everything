import { Container, Skeleton, MantineStyleProp } from '@mantine/core';
import { ComponentProps, useCallback, useState } from 'react';

export interface ResponsiveImageProps extends ComponentProps<'img'> { }

export function ResponsiveImage(props: ResponsiveImageProps) {
    const { alt, src } = props;
    const [size, setSize] = useState<{w: number, h: number}>({ w: 0, h: 0 });
    const [isLoaded, setLoaded] = useState<boolean>(false);

    const measuredRef = useCallback((node: HTMLImageElement) => {
        if (node !== null && isLoaded) {
            setSize({ w: node.naturalWidth, h: node.naturalHeight });
        }
    }, [isLoaded]);

    // if Image width larger than height which mean it need to change flex direction to vertical align
    const needVerticalAlign: MantineStyleProp = size.w > size.h
        ? {
            display:        'flex',
            flexDirection:  'column',
            justifyContent: 'center',
        } : {};

    return (
        <Container h="100%" style={needVerticalAlign}>
            <img
                ref={measuredRef}
                onLoad={() => setLoaded(true)}
                alt={alt}
                src={src}
                hidden={(size.w + size.h) === 0}
                width={(size.w < size.h) ? 'inherit' : '100%'}
                height={(size.w > size.h) ? 'inherit' : '100%'}
                // eslint-disable-next-line react/jsx-props-no-spreading
                {...props}
            />
            { ((size.w + size.h) === 0) && <Skeleton w="100%" h="100%" /> }
        </Container>
    );
}
