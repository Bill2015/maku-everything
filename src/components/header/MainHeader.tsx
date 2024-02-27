import { PropsWithChildren, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import {
    Button, Divider, Group, Image, Menu, Select, Tooltip, UnstyledButton,
} from '@mantine/core';
import { LuImport } from 'react-icons/lu';
import { BsGear } from 'react-icons/bs';
import { AiOutlineThunderbolt } from 'react-icons/ai';
import { MdLanguage } from 'react-icons/md';
import { useConfigRedux } from '@store/global';
import { SupportLangs, defaultLang, SupportLangsType } from '@modules/i18next';
import { useHomeNavigate } from '@router/navigateHook';
import { useImportCategoryModal } from '@store/modal';

import Logo from '@assets/logo.png';

import classes from './MainHeader.module.scss';

function MenuButton(props: PropsWithChildren) {
    const { children } = props;
    return (
        <Menu.Target>
            <Button classNames={{ root: classes.btn }}>
                <Group p={0} gap={3}>
                    {children}
                </Group>
            </Button>
        </Menu.Target>

    );
}
function MenuDropDown(props: PropsWithChildren) {
    const { children } = props;
    return (
        <Menu.Dropdown>
            {children}
        </Menu.Dropdown>
    );
}

function HeaderMenuItem(props: PropsWithChildren) {
    const { children } = props;
    return (
        <Menu
            withArrow
            shadow="md"
            position="bottom-start"
            trigger="hover"
            arrowOffset={5}
            offset={0}
        >
            {children}
        </Menu>
    );
}

export function MainHeader() {
    const { t, i18n } = useTranslation('common', { keyPrefix: 'Header.MainHeader' });
    const { config, updateConfig } = useConfigRedux();
    const navigateToHome = useHomeNavigate();
    const [_, { open }] = useImportCategoryModal();

    useEffect(() => {
        if (config) {
            i18n.changeLanguage(config.lang);
        }
    }, [i18n, config]);

    return (
        <Group px="sm" gap={5}>
            <Tooltip label="Home" position="right">
                <UnstyledButton onClick={navigateToHome} pl={8} pr={15}>
                    <Image width="30px" height="30px" src={Logo} />
                </UnstyledButton>
            </Tooltip>

            {/* Action section */}
            <Divider orientation="vertical" opacity="0.5" />
            <HeaderMenuItem>
                <MenuButton>
                    <AiOutlineThunderbolt />
                    {t('actions')}
                </MenuButton>

                <MenuDropDown>
                    <Menu.Item fz="xs" pr={50} leftSection={<LuImport />} onClick={() => open()}>
                        {t('import_category')}
                    </Menu.Item>
                </MenuDropDown>
            </HeaderMenuItem>

            {/* Setting section */}
            <Divider orientation="vertical" opacity="0.5" />
            <HeaderMenuItem>
                <MenuButton>
                    <BsGear />
                    {t('settings')}
                </MenuButton>
            </HeaderMenuItem>

            <Select
                variant="unstyled"
                className={classes.langSelect}
                classNames={{ input: classes.langInput }}
                withCheckIcon={false}
                allowDeselect={false}
                leftSection={<MdLanguage />}
                leftSectionPointerEvents="none"
                rightSectionPointerEvents="none"
                rightSectionWidth={0}
                styles={{ dropdown: { maxHeight: 200, overflowY: 'auto' } }}
                defaultValue={config?.lang ?? defaultLang.key}
                value={config?.lang}
                data={Object.values(SupportLangs).map((val) => ({
                    value:    val.key,
                    label:    val.displayName,
                    disabled: i18n.language === val.key,
                }))}
                onChange={(val) => {
                    updateConfig({ lang: val as SupportLangsType });
                }}
            />
        </Group>
    );
}
