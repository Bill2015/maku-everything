import { useTranslation } from 'react-i18next';
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
    const { t } = useTranslation('pages', { keyPrefix: 'CategoryList.CategoryCardMenu' });
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
                    {t('settings')}
                </Menu.Item>
                <Menu.Item leftSection={<LuPin />}>
                    {t('pin')}
                </Menu.Item>
                <Menu.Item leftSection={<CgExport />} onClick={onExportclick}>
                    {t('export')}
                </Menu.Item>
            </Menu.Dropdown>
        </Menu>
    );
}
