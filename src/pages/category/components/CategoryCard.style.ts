import { createStyles } from '@mantine/core';

export const useCategoryCardStyle = createStyles((theme) => ({
    card: {
        borderRadius:   theme.radius.md,
        display:        'flex',
        alignItems:     'center',
        justifyContent: 'center',
        color:          theme.colorScheme === 'dark' ? theme.white : theme.black,
        opacity:        0.85,

        '&:hover': {
            opacity:         1,
            backgroundColor: theme.fn.lighten(
                theme.fn.variant({ variant: 'filled', color: theme.primaryColor }).background!,
                0.1,
            ),
        },
    },

    active: {
        opacity:      1,
        '&, &:hover': {
            backgroundColor: theme.fn.lighten(
                theme.fn.variant({ variant: 'filled', color: theme.primaryColor }).background!,
                0.15,
            ),
        },
    },
}));
