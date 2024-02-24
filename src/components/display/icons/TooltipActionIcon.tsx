/* eslint-disable react/jsx-props-no-spreading */
import { PropsWithChildren } from 'react';

import { ActionIcon, ActionIconProps, ElementProps, Tooltip, TooltipProps } from '@mantine/core';

import classes from './TooltipActionIcon.module.scss';

export interface TooltipActionIconProps extends ActionIconProps, ElementProps<'button', keyof ActionIconProps>, PropsWithChildren {
    tooltipProps?: TooltipProps & ElementProps<'div', keyof TooltipProps>;

    label: string;

    onTooltipChange?: (opened: boolean) => void;
}

/**
 * According URL host to determin which icon will be showing */
export function TooltipActionIcon(props: TooltipActionIconProps) {
    const {
        label,
        children,
        tooltipProps,
        onTooltipChange,
        ...actionIconProps
    } = props;

    return (
        <Tooltip
            withArrow
            label={label}
            classNames={{ tooltip: classes.tooltip }}
            offset={10}
            {...tooltipProps}
        >
            <ActionIcon
                classNames={{ root: classes.ActionIconRoot }}
                {...actionIconProps}
                onMouseEnter={() => onTooltipChange && onTooltipChange(true)}
                onMouseLeave={() => onTooltipChange && onTooltipChange(false)}
            >
                {children}
            </ActionIcon>
        </Tooltip>
    );
}
