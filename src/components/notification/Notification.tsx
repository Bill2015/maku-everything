import { Text } from '@mantine/core';

import { NotificationData, showNotification as mantineNoticfication } from '@mantine/notifications';
import { ReactNode } from 'react';

type NotificationType = 'info' | 'error' | 'success';

export type NotificationOption = Omit<NotificationData, 'title'|'message'|'color'>;

export function showNotification(title: string | ReactNode, message: string | ReactNode, varient: NotificationType = 'info', options: NotificationOption = {}) {
    const defaultValue: NotificationData = {
        withBorder:      true,
        withCloseButton: true,
        title:           title,
        message:         message,
        autoClose:       2000,
        ...options,
    };
    switch (varient) {
    case 'info':
        mantineNoticfication({
            ...defaultValue,
            color:   'blue',
            message: (<Text c="white">{message}</Text>),
            bg:      '#5385e5',
        });
        break;
    case 'success':
        mantineNoticfication({
            ...defaultValue,
            color:   'lime',
            message: (<Text c="white">{message}</Text>),
            bg:      '#3cab3a',
        });
        break;
    case 'error':
        mantineNoticfication({
            ...defaultValue,
            color:   'red',
            message: (<Text c="white">{message}</Text>),
            bg:      '#fc4d4d',
        });
        break;
    // eslint-disable-next-line no-fallthrough
    default:
        break;
    }
}
