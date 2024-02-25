/* eslint-disable react/jsx-props-no-spreading */
import { ResponsiveImage, ResponsiveImageProps } from '@components/display/ResponsiveImage';
import UnsupportImage from '@assets/icons/not-supported-icon.png';

export interface LocalUnsupportThumbnailProps extends Omit<ResponsiveImageProps, 'src' | 'ref'> {
    mediaType: string;
}

export function LocalUnsupportThumbnail(props: LocalUnsupportThumbnailProps) {
    const { mediaType, ...imgProps } = props;

    return (<ResponsiveImage src={UnsupportImage} {...imgProps} alt={mediaType} />);
}
