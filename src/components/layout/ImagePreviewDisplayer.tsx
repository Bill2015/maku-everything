import React, { ReactElement } from 'react';
import { Flex, ImageProps } from '@mantine/core';

import classes from './ImagePreviewDisplayer.module.scss';

export interface ImagePreviewFlexProps {
    children: ReactElement<ImageProps>[]
}

export function ImagePreviewFlex(props: ImagePreviewFlexProps) {
    const { children } = props;

    const newChildren = React.Children.map(children, (val) => {
        if (React.isValidElement<ImageProps>(val)) {
            return React.cloneElement(val, {
                ...val.props,
                className: `${classes.previewImg} ${val.props.className || ''}`,
            });
        }
        throw new Error('Only can contain image element');
    });

    return (
        <Flex h={300} display="flex" align="center" justify="center">
            {newChildren}
        </Flex>
    );
}
