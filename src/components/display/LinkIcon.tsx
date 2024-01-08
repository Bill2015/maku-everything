import { open } from '@tauri-apps/api/shell';

import { ActionIcon, ActionIconProps, Tooltip } from '@mantine/core';
import { IconType } from 'react-icons';
import { FaYoutube } from 'react-icons/fa';
import { FaLink } from 'react-icons/fa6';
import { useSnackbar } from 'notistack';

import classes from './LinkIcon.module.scss';

const URL_ICON_MAPPER = new Map<string, IconType>();
URL_ICON_MAPPER.set('www.youtube.com', FaYoutube);

export interface LinkIconProps extends ActionIconProps {
    /** Host of URL, for example: **www.youtube.com**  */
    host: string;

    /** full URL string */
    url: string;
}

/**
 * According URL host to determin which icon will be showing */
export function LinkIcon(props: LinkIconProps) {
    const { host, url, ...actionIconProps } = props;
    const { enqueueSnackbar } = useSnackbar();

    const IconElement = (() => {
        if (URL_ICON_MAPPER.has(host)) {
            return URL_ICON_MAPPER.get(host)!;
        }
        // defualt icon
        return FaLink;
    })();

    return (
        <Tooltip
            withArrow
            label={`↖️ ${url}`}
            classNames={{ tooltip: classes.tooltip }}
            offset={10}
        >
            <ActionIcon
                classNames={{ root: classes.ActionIconRoot }}
                // eslint-disable-next-line react/jsx-props-no-spreading
                {...actionIconProps}
                onClick={() => {
                    open(url)
                        .catch(() => {
                            enqueueSnackbar('Invalid URL', { variant: 'error' });
                        });
                }}
            >
                <IconElement />
            </ActionIcon>
        </Tooltip>
    );
}
