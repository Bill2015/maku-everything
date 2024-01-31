import { open } from '@tauri-apps/api/shell';

import { ActionIcon, ActionIconProps, Tooltip } from '@mantine/core';
import { IconType } from 'react-icons';
import { FaYoutube } from 'react-icons/fa';
import { FaLink } from 'react-icons/fa6';

import { showNotification } from '@components/notification';
import { UrlHost } from '@declares/variables';

import classes from './LinkIcon.module.scss';

const URL_ICON_MAPPER = new Map<string, IconType>();
URL_ICON_MAPPER.set(UrlHost.Youtube, FaYoutube);

export interface LinkIconProps extends ActionIconProps {
    /** URLs */
    url: {
        full: string;

        host: string;
    };
}

/**
 * According URL host to determin which icon will be showing */
export function LinkIcon(props: LinkIconProps) {
    const { url, ...actionIconProps } = props;

    const IconElement = (() => {
        if (URL_ICON_MAPPER.has(url.host)) {
            return URL_ICON_MAPPER.get(url.host)!;
        }
        // defualt icon
        return FaLink;
    })();

    return (
        <Tooltip
            withArrow
            label={`↖️ ${url.full}`}
            classNames={{ tooltip: classes.tooltip }}
            offset={10}
        >
            <ActionIcon
                classNames={{ root: classes.ActionIconRoot }}
                // eslint-disable-next-line react/jsx-props-no-spreading
                {...actionIconProps}
                onClick={() => {
                    open(url.full)
                        .catch(() => {
                            showNotification('Invalid URL', url.full, 'error');
                        });
                }}
            >
                <IconElement />
            </ActionIcon>
        </Tooltip>
    );
}
