/* eslint-disable react/jsx-props-no-spreading */
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { ResponsiveImage } from '@components/display';
import { LocalThumbnailProps } from './types';

export function LocalImageThumbnail(props: LocalThumbnailProps) {
    const { filePath, ...imgProps } = props;
    return <ResponsiveImage src={convertFileSrc(filePath)} {...imgProps} />;
}
