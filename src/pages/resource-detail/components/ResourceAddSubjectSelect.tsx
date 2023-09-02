import { useState, useMemo, useRef, useCallback } from 'react';
import { Group, Text } from '@mantine/core';
import { SubjectResDto } from '@api/subject';
import { SubjectSelect, SubjectSelectItem } from '@components/input';
import { TagQuery, TagResDto } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';
import { ResourceTagSelect } from './ResourceTagSelect';

export interface ResourceAddSubjectSelectProps {
    subjects: SubjectResDto[];

    exclude: Set<string>;

    onSelectNewTag: (value: TagResDto) => void;
}

export function ResourceAddSubjectSelect(props: ResourceAddSubjectSelectProps) {
    const { subjects, exclude, onSelectNewTag } = props;
    const inputRef = useRef<HTMLInputElement>(null);
    const { activeCategory } = useActiveCategoryRedux();
    const [subjectValue, setSubjectValue] = useState<string>('');
    const [selectedSubject, setSelectedSubject] = useState<SubjectSelectItem | null>(null);
    const { data: subjectTags } = TagQuery.useGetSubjectTags(activeCategory!.id, selectedSubject && selectedSubject!.id);

    const visibleSubject = useMemo(() => subjects.filter((val) => !exclude.has(val.id)), [subjects, exclude]);

    const handleTagSelect = (value: TagResDto | undefined) => {
        if (value) {
            setSelectedSubject(null);
            setSubjectValue('');
            onSelectNewTag(value);
        }
    };

    const handleSubjectItemSelect = useCallback((data: SubjectSelectItem) => {
        setSelectedSubject(data);
        setTimeout(() => {
            inputRef.current?.focus();
        }, 10);
    }, []);

    return (
        <>
            <SubjectSelect
                value={subjectValue}
                display={selectedSubject ? 'none' : 'unset'}
                onItemSelect={handleSubjectItemSelect}
                subjects={visibleSubject}
            />
            <Group display={selectedSubject ? 'flex' : 'none'} spacing="xs">
                <Text fw="bolder">
                    {selectedSubject ? selectedSubject.value : ''}
                    :
                </Text>
                <ResourceTagSelect
                    ref={inputRef}
                    rightSectionWidth={0}
                    data={subjectTags}
                    onKeyDown={(e) => {
                        if (e.key === 'Backspace' && !(inputRef.current!.value)) {
                            setSelectedSubject(null);
                            setSubjectValue('');
                        }
                    }}
                    onItemSelect={handleTagSelect}
                />
            </Group>
        </>
    );
}
