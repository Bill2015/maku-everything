/* eslint-disable react/jsx-props-no-spreading */
import { Modal, ModalProps } from '@mantine/core';
import classes from './BaseModal.module.scss';

export interface BaseModalProps extends ModalProps { }

export function BaseModal(props: BaseModalProps) {
    const { children, ...modalProps } = props;

    return (
        <Modal centered {...modalProps} classNames={{ ...classes, ...modalProps.classNames }}>
            {children}
        </Modal>
    );
}
