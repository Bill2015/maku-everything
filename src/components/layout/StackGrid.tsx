import React, { CSSProperties, PropsWithChildren, useEffect, useRef, useState } from 'react';

import { Box } from '@mantine/core';

import classes from './StackGrid.module.scss';

interface StackGridItemProps extends PropsWithChildren { }

function StackGridItem(props: StackGridItemProps) {
    const { children } = props;

    const ref = useRef<HTMLDivElement>(null);
    const [size, setSize] = useState<number>(0);

    useEffect(() => {
        if (!ref || !ref.current) {
            return;
        }

        // observed the children size change and make parent update
        const resizeObserver = new ResizeObserver((event) => {
            setSize(Math.floor(event[0].borderBoxSize[0].blockSize / 10) + 2);
        });

        resizeObserver.observe(ref.current!.firstElementChild!);
        return () => {
            resizeObserver.disconnect();
        };
    }, []);

    return (
        <Box ref={ref} className={classes.stackItem} style={{ gridRowEnd: `span ${size}` }}>
            {children}
        </Box>
    );
}

interface StackGridProps extends PropsWithChildren {
    w: number | string;
    justify?: CSSProperties['justifyContent'];
}

/**
 * A pinterest like layout. \
 * References: https://www.youtube.com/watch?v=baBvJDmziGQ */
export function StackGrid(props: StackGridProps) {
    const {
        children,
        w,
        justify = 'center',
    } = props;

    return (
        <Box
            className={classes.stackGridContainer}
            style={{
                gridTemplateColumns: `repeat(auto-fill, ${Number.isInteger(w) ? `${w}px` : w})`,
                justifyContent:      justify,
            }}
        >
            {React.Children.map(children, (child) => (<StackGridItem>{child}</StackGridItem>))}
        </Box>
    );
}
