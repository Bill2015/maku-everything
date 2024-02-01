import { ActionIcon, Menu } from '@mantine/core';
import { MdOutlineMoreVert } from 'react-icons/md';
import { CgExport } from 'react-icons/cg';
import { IoSettingsOutline } from 'react-icons/io5';
import { LuPin } from 'react-icons/lu';

import classes from './CategoryCardMenu.module.scss';

export interface CategoryCardMenuProps {
    name: string;

    onExportclick: () => void
}

export function CategoryCardMenu(props: CategoryCardMenuProps) {
    const { name, onExportclick } = props;
    return (
        <Menu
            shadow="md"
            withArrow
            loop
            width={200}
            position="bottom-start"
            arrowSize={14}
            offset={0}
        >
            <Menu.Target>
                <ActionIcon pos="absolute" variant="outline" classNames={{ root: classes.menuBtn }}>
                    <MdOutlineMoreVert />
                </ActionIcon>
            </Menu.Target>
            <Menu.Dropdown>
                <Menu.Label>
                    {name}
                </Menu.Label>
                <Menu.Item leftSection={<IoSettingsOutline />}>
                    Settings
                </Menu.Item>
                <Menu.Item leftSection={<LuPin />}>
                    Pin
                </Menu.Item>
                <Menu.Item leftSection={<CgExport />} onClick={onExportclick}>
                    Export
                </Menu.Item>
            </Menu.Dropdown>
        </Menu>
    );
}
