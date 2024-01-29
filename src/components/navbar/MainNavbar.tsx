import React, { useState } from 'react';
import { Stack, UnstyledButton, Tooltip } from '@mantine/core';
import { IconBaseProps } from 'react-icons/lib';
import { BsGear, BsMailbox } from 'react-icons/bs';

import navbarClasses from './MainNavbar.module.scss';

interface NavbarLinkProps {
    icon: React.FC<IconBaseProps>;
    label: string;
    active: boolean;
    onClick?: () => void;
}

function NavbarLink(props: NavbarLinkProps) {
    const { icon: Icon, label, active, onClick } = props;

    return (
        <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
            <UnstyledButton onClick={onClick} className={navbarClasses.link}>
                <Icon size="1.5rem" stroke="1.5" />
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
        <Stack style={{ alignItems: 'center' }} gap={5}>
            {/* {navbarItem} */}
        </Stack>
    );
}
