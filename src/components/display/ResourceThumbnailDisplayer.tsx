/* eslint-disable react/jsx-props-no-spreading */
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { ResourceResDto } from '@api/resource';

import { YoutubeThumbnail } from './YoutubeThunbnail';
import { ResponsiveImage, ResponsiveImageProps } from './ResponsiveImage';

interface ResourceThumbnailDisplayerProps extends Omit<ResponsiveImageProps, 'alt'> {
    data: ResourceResDto;
}

export function ResourceThumbnailDisplayer(props: ResourceThumbnailDisplayerProps) {
    const { data, ...imgProps } = props;

    return (
        data.file === null
            ? <YoutubeThumbnail url={data.url!.full} alt={data.name} {...imgProps} />
            : <ResponsiveImage src={convertFileSrc(data.root_path + data.file!.path)} alt={data.name} {...imgProps} />
    );
}
