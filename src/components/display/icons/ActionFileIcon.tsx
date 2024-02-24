/* eslint-disable react/jsx-props-no-spreading */
import { useCallback } from 'react';
import { FcOpenedFolder } from 'react-icons/fc';

import { ResourceMutation } from '@api/resource';

import { TooltipActionIcon, TooltipActionIconProps } from './TooltipActionIcon';

export interface ActionFileIconProps extends Omit<TooltipActionIconProps, 'label'> {
    filePath: string | null;
}

/**
 * According URL host to determin which icon will be showing */
export function ActionFileIcon(props: ActionFileIconProps) {
    const {
        filePath,
        ...actionIconProps
    } = props;

    const exporeFile = ResourceMutation.useExporeFile();

    const handleExporeClick = useCallback(() => {
        if (filePath) {
            exporeFile.mutateAsync(filePath);
        }
    }, [exporeFile, filePath]);

    return (
        <TooltipActionIcon
            label={`↖️ ${filePath}`}
            onClick={handleExporeClick}
            {...actionIconProps}
        >
            <FcOpenedFolder />
        </TooltipActionIcon>
    );
}
