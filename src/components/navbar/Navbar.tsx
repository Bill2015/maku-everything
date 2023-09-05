import React, { useState } from 'react';
import { Navbar, Stack, UnstyledButton, Tooltip, Image } from '@mantine/core';
import { IconBaseProps } from 'react-icons/lib';
import { BsGear, BsMailbox } from 'react-icons/bs';
import Logo from '@assets/logo.png';
import { useHomeNavigate } from '@router/navigateHook';

import { useNavbarStyles } from './Navbar.style';

interface NavbarLinkProps {
    icon: React.FC<IconBaseProps>;
    label: string;
    active: boolean;
    onClick?: () => void;
}

function NavbarLink(props: NavbarLinkProps) {
    const { classes, cx } = useNavbarStyles();
    const { icon: Icon, label, active, onClick } = props;

    return (
        <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
            <UnstyledButton onClick={onClick} className={cx(classes.link, { [classes.active]: active })}>
                <Icon size="1.2rem" stroke="1.5" />
            </UnstyledButton>
        </Tooltip>
    );
}

const mockdata = [
    { icon: BsGear, label: 'Setting' },
    { icon: BsMailbox, label: 'Contact' },
];

export function MainNavbar() {
    const [active, setActive] = useState(2);
    const navigateToHome = useHomeNavigate();

    const navbarItem = mockdata.map((val, index) => (
        <NavbarLink
            icon={val.icon}
            label={val.label}
            key={val.label}
            active={index === active}
            onClick={() => {
                setActive(index);
            }}
        />
    ));

    return (
        <Navbar height={750} width={{ base: 50 }} p="xs" pl={0} pr={0}>
            <Navbar.Section>
                <Tooltip label="Maku" position="right" transitionProps={{ duration: 0 }}>
                    <UnstyledButton onClick={navigateToHome}>
                        <Image p={5} src={Logo} />
                    </UnstyledButton>
                </Tooltip>
            </Navbar.Section>
            <Navbar.Section grow>
                <Stack justify="center" spacing={2}>
                    {navbarItem}
                </Stack>
            </Navbar.Section>
        </Navbar>
    );
}
