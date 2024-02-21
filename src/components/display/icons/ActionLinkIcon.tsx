/* eslint-disable react/jsx-props-no-spreading */
import { open } from '@tauri-apps/api/shell';

import { IconType } from 'react-icons';
import { FaYoutube } from 'react-icons/fa';
import { FaLink } from 'react-icons/fa6';

import { showNotification } from '@components/notification';
import { UrlHost } from '@declares/variables';

import classes from './ActionLinkIcon.module.scss';
import { TooltipActionIcon, TooltipActionIconProps } from './TooltipActionIcon';

const URL_ICON_MAPPER = new Map<string, IconType>();
URL_ICON_MAPPER.set(UrlHost.Youtube, FaYoutube);

export interface ActionLinkIconProps extends Omit<TooltipActionIconProps, 'label'> {
    /** URLs */
    url: {
        full: string;

        host: string;
    };
}

/**
 * According URL host to determin which icon will be showing */
export function ActionLinkIcon(props: ActionLinkIconProps) {
    const {
        url,
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
        <TooltipActionIcon
            label={`↖️ ${url.full}`}
            classNames={{ root: classes.ActionIconRoot }}
            onClick={() => {
                open(url.full)
                    .catch(() => {
                        showNotification('Invalid URL', url.full, 'error');
                    });
            }}
            {...actionIconProps}
        >
            <IconElement />
        </TooltipActionIcon>
    );
}
