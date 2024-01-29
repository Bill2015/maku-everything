import { PropsWithChildren } from 'react';
import { Button, Divider, Group, Image, Menu, Tooltip, UnstyledButton } from '@mantine/core';
import { LuImport } from 'react-icons/lu';
import { BsGear } from 'react-icons/bs';
import { AiOutlineThunderbolt } from 'react-icons/ai';
import { useHomeNavigate } from '@router/navigateHook';
import { useImportCategoryModel } from '@store/modal';

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
    const navigateToHome = useHomeNavigate();
    const { open } = useImportCategoryModel();

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
                    Actions
                </MenuButton>

                <MenuDropDown>
                    <Menu.Item fz="xs" pr={50} leftSection={<LuImport />} onClick={() => open()}>
                        Import Category
                    </Menu.Item>
                </MenuDropDown>
            </HeaderMenuItem>

            {/* Setting section */}
            <Divider orientation="vertical" opacity="0.5" />
            <HeaderMenuItem>
                <MenuButton>
                    <BsGear />
                    Settings
                </MenuButton>
            </HeaderMenuItem>
        </Group>
    );
}
