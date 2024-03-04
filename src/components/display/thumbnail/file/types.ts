import { ResponsiveImageProps } from '@components/display/ResponsiveImage';

export interface LocalThumbnailProps extends Omit<ResponsiveImageProps, 'src' | 'ref'> {
    filePath: string;
}
