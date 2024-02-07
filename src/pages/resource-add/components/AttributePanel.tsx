import { EditableText } from '@components/display';
import { InputWrapper, Stack } from '@mantine/core';

export interface AttributePanelProps {

}

export function AttributePanel() {
    return (
        <Stack>
            <InputWrapper label="name">
                {/* <EditableText /> */}
            </InputWrapper>
        </Stack>
    );
}
