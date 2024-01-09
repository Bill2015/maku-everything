import { Group, Text } from '@mantine/core';
import { MdOutlineWebAssetOff } from 'react-icons/md';

export type WebEmbedProps = {
    /** URL of the web */
    url: string;

    /** Name of web */
    name: string;
}

// eslint-disable-next-line no-undef, func-call-spacing, no-spaced-func
const RegistedWebEmbed = new Map<string, ((props: WebEmbedProps) => JSX.Element)>();

/**
 * Registe the web embed Component \
 * It can be very quick adding new Web embed.
 *
 * Example:
 * ```typescript
 *  // adding youtube embed
 *  RegisteWebembedComponent('www.youtube.com', YoutubeVideoembed)
 * ```
 * @param host Host of the Web
 * @param components Display component */
// eslint-disable-next-line no-undef
export function RegisteWebEmbedComponent(host: string, components: ((props: WebEmbedProps) => JSX.Element)) {
    if (RegistedWebEmbed.has(host)) {
        // eslint-disable-next-line no-console
        console.error(RegistedWebEmbed.get(host));
        throw Error('Host Already Registed');
    }

    RegistedWebEmbed.set(host, components);
}

export interface WebEmbedEntryProps extends WebEmbedProps {
    host: string;
}

/**
 * Determine which web host needs to use which components to display it */
export function WebEmbedDisplayer(props: WebEmbedEntryProps) {
    const { host, name, url } = props;

    if (RegistedWebEmbed.has(host)) {
        const EmbedElement = RegistedWebEmbed.get(host)!;
        return <EmbedElement url={url} name={name} />;
    }

    return (
        <Group justify="center">
            <MdOutlineWebAssetOff style={{ width: '100%', height: '100%' }} />
            <Text fw={800}>Unsupport URL Embed</Text>
        </Group>
    );
}
