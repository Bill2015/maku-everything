import React, { forwardRef } from 'react';
import { Button, ButtonProps } from '@mantine/core';
import { BsArrowReturnLeft } from 'react-icons/bs';
import { useNavigate } from 'react-router-dom';

export interface ReturnButtonProps extends ButtonProps {
    onClick?: (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void;
}

export const ReturnButton = forwardRef<HTMLButtonElement, ReturnButtonProps>((props, ref) => {
    const { onClick, ...btnProps } = props;

    const naviagte = useNavigate();
    return (
        <Button
            ref={ref}
            onClick={(e) => {
                if (onClick) {
                    onClick(e);
                }
                naviagte(-1);
            }}
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...btnProps}
        >
            <BsArrowReturnLeft />
        </Button>
    );
});
