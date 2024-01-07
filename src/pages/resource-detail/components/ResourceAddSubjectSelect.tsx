import { useState, useMemo, useRef, useCallback } from 'react';
import { Group, Text } from '@mantine/core';
import { SubjectResDto } from '@api/subject';
import { SubjectSelect, SubjectSelectItemData } from '@components/input';
import { TagQuery, TagResDto } from '@api/tag';
import { ResourceTagSelect } from './ResourceTagSelect';

export interface ResourceAddSubjectSelectProps {
    subjects: SubjectResDto[];

    exclude: Set<string>;

    onSelectNewTag: (value: TagResDto) => void;
}

export function ResourceAddSubjectSelect(props: ResourceAddSubjectSelectProps) {
    const { subjects, exclude, onSelectNewTag } = props;
    const tagInputRef = useRef<HTMLInputElement>(null);
    const subjectInputRef = useRef<HTMLInputElement>(null);
    const [tagValue, setTagValue] = useState<string>('');
    const [tagSearchValue, setTagSearchValue] = useState<string>('');

    const [selectedSubject, setSelectedSubject] = useState<SubjectSelectItemData | null>(null);
    const { data: subjectTags } = TagQuery.useGetSubjectTags(selectedSubject && selectedSubject!.id);

    const visibleSubject = useMemo(() => subjects.filter((val) => !exclude.has(val.id)), [subjects, exclude]);

    const handleTagSelect = (value: TagResDto | undefined) => {
        if (value) {
            setSelectedSubject(null);
            onSelectNewTag(value);
        }
    };

    const handleSubjectItemSelect = useCallback((data: SubjectSelectItemData) => {
        setSelectedSubject(data);
        setTagValue('');
        setTagSearchValue('');
        setTimeout(() => {
            tagInputRef.current?.focus();
        }, 10);
    }, []);

    return (
        <>
            <SubjectSelect
                inputRef={subjectInputRef}
                onItemSelect={handleSubjectItemSelect}
                hidden={!!selectedSubject}
                subjects={visibleSubject}
            />
            <Group display={selectedSubject ? 'flex' : 'none'} gap="xs">
                <Text fw="bolder">
                    {selectedSubject ? selectedSubject.value : ''}
                    :
                </Text>
                <ResourceTagSelect
                    ref={tagInputRef}
                    rightSectionWidth={0}
                    data={subjectTags}
                    searchValue={tagValue}
                    value={tagSearchValue}
                    onKeyDown={(e) => {
                        if (e.key === 'Backspace' && !(tagValue)) {
                            setSelectedSubject(null);
                            setTimeout(() => subjectInputRef.current?.focus(), 10);
                        }
                    }}
                    onSearchChange={(e) => setTagValue(e)}
                    onItemSelect={handleTagSelect}
                />
            </Group>
        </>
    );
}
