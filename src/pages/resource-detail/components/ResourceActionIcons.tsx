/* eslint-disable react/jsx-props-no-spreading */
import { ActionFileIcon, TooltipActionIcon } from '@components/display';
import { LuFileType2 } from 'react-icons/lu';

function Rename({ onClick }: { onClick: () => void }) {
    return (
        <TooltipActionIcon
            label="Rename the file by current name"
            color="gold"
            fz="1.25em"
            pos="absolute"
            right="10px"
            top="30px"
            variant="subtle"
            style={{ zIndex: 5 }}
            onClick={onClick}
        >
            <LuFileType2 />
        </TooltipActionIcon>
    );
}

function Explore({ filePath }: { filePath: string }) {
    return (
        <ActionFileIcon
            filePath={filePath}
            pos="absolute"
            top="0px"
            right="10px"
            variant="subtle"
            p={0}
            fz="1.75em"
        />
    );
}

export function ResourceActionIcons() {}

ResourceActionIcons.Rename = Rename;
ResourceActionIcons.Explore = Explore;
