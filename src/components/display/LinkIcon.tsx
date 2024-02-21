/* eslint-disable react/jsx-props-no-spreading */
import { open } from '@tauri-apps/api/shell';

import { ActionIcon, ActionIconProps, ElementProps, Tooltip, TooltipProps } from '@mantine/core';
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
    tooltipProps?: TooltipProps & ElementProps<'div', keyof TooltipProps>;

    onTooltipOpen?: () => void;
    onTooltipClose?: () => void;
}

/**
 * According URL host to determin which icon will be showing */
export function LinkIcon(props: LinkIconProps) {
    const {
        url,
        tooltipProps,
        onTooltipOpen,
        onTooltipClose,
        ...actionIconProps
    } = props;

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
            {...tooltipProps}
        >
            <ActionIcon
                classNames={{ root: classes.ActionIconRoot }}
                {...actionIconProps}
                onClick={() => {
                    open(url.full)
                        .catch(() => {
                            showNotification('Invalid URL', url.full, 'error');
                        });
                }}
                onMouseEnter={() => onTooltipOpen && onTooltipOpen()}
                onMouseLeave={() => onTooltipClose && onTooltipClose()}
            >
                <IconElement />
            </ActionIcon>
        </Tooltip>
    );
}
