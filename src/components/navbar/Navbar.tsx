import React, { useState } from 'react';
import { Navbar, Stack, UnstyledButton, Tooltip } from '@mantine/core';
import { IconBaseProps } from 'react-icons/lib';
import { HiHome } from 'react-icons/hi';
import { BsGear } from 'react-icons/bs';
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
    {
        icon: HiHome, label: 'Home',
    },
    {
        icon: BsGear, label: 'Setting',
    },
];

export function MainNavbar() {
    const [active, setActive] = useState(2);

    const navbarItem = mockdata.map((val, index) => (
        <NavbarLink
            icon={val.icon}
            label={val.label}
            key={val.label}
            active={index === active}
            onClick={() => setActive(index)}
        />
    ));

    return (
        <Navbar height={750} width={{ base: 80 }} p="md">
            <Navbar.Section grow mt={50}>
                <Stack justify="center" spacing={0}>
                    {navbarItem}
                </Stack>
            </Navbar.Section>
        </Navbar>
    );
}
