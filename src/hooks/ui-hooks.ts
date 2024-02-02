import { useMemo } from 'react';

export const useBackGroundImage = ((useBackgoundImg: boolean, loaded: boolean, attr: { src: string, alt: string }) => {
    const { src, alt } = attr;

    return useMemo(() => {
        if (useBackgoundImg && loaded) {
            return { style: { backgroundImage: `url("${src}")` } };
        }
        return { src: src, alt: alt };
    }, [useBackgoundImg, loaded, src, alt]);
});
