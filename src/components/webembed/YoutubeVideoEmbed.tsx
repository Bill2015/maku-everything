import { UrlHost, YoutubePrefix } from '@declares/variables';
import { Center } from '@mantine/core';
import { CiVideoOff } from 'react-icons/ci';

import { RegisteWebEmbedComponent, WebEmbedProps } from './WebEmbedDisplayer';

export interface YoutubeVideoEmbedProps extends WebEmbedProps {
    /** Youtube **video** or **Short** URL */
    url: string;
}

/**
 * Embed Youtube Video \
 * Support normal **videos** and **Shorts** */
export function YoutubeVideoEmbed(props: YoutubeVideoEmbedProps) {
    const { url, name } = props;

    const youtubeId = (() => {
        if (url.startsWith(YoutubePrefix.NormalVideo)) {
            return url.substring(YoutubePrefix.NormalVideo.length);
        }
        if (url.startsWith(YoutubePrefix.ShortVideo)) {
            return url.substring(YoutubePrefix.ShortVideo.length);
        }
        return '';
    })();

    if (!youtubeId) {
        return (
            <Center w="100%" h="100%">
                <CiVideoOff style={{ width: '100%', height: '100%' }} />
            </Center>
        );
    }

    return (
        <iframe
            width="100%"
            height="315"
            src={`https://www.youtube.com/embed/${youtubeId}`}
            title={name}
            frameBorder="0"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
            allowFullScreen
        />
    );
}

RegisteWebEmbedComponent(UrlHost.Youtube, YoutubeVideoEmbed);
