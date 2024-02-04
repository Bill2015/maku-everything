import { open } from '@tauri-apps/api/dialog';
import { ActionIcon, ElementProps, Flex, Input, InputProps } from '@mantine/core';
import { FcOpenedFolder } from 'react-icons/fc';
import { useTranslation } from 'react-i18next';

export interface PathInputProps extends Omit<InputProps & ElementProps<'input', keyof InputProps>, 'value'|'onChange'> {
    value: string;

    directory?: boolean;

    onChange: (val: string) => void;
}

export function PathInput(props: PathInputProps) {
    const { value, onChange, directory = false, ...inputProps } = props;
    const { t } = useTranslation('common', { keyPrefix: 'Input.PathInput' });

    const handleClick = async () => {
        const selectedPath = await open({
            multiple:  false,
            directory: directory,
        }) as string;
        onChange(selectedPath);
    };

    return (
        <Flex gap={5}>
            <Input
                flex={1}
                value={value}
                title={value}
                onChange={(e) => onChange(e.currentTarget.value)}
                // eslint-disable-next-line react/jsx-props-no-spreading
                {...inputProps}
            />
            <ActionIcon
                w="2.25rem"
                h="auto"
                variant="outline"
                fz="1.5rem"
                role="button"
                style={{ borderColor: 'var(--mantine-color-default-border)' }}
                title={t('file_button')}
                onClick={handleClick}
            >
                <FcOpenedFolder />
            </ActionIcon>
        </Flex>
    );
}
