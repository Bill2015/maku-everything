/* eslint-disable no-await-in-loop */
import { Affix, Button, rem } from '@mantine/core';
import { CategoryAPI, CategoryMutation } from '@api/category';
import { SubjectAPI, SubjectMutation } from '@api/subject';
import { TagMutation } from '@api/tag';

import { showNotification } from '@components/notification';
import { initialData } from './data';
import { importDefaultData } from './data2';

export function Initializer() {
    const createCategory = CategoryMutation.useCreate();
    const createSubject = SubjectMutation.useCreate();
    const tagMutation = TagMutation.useCreate();

    const importCategory = CategoryMutation.useImport();

    async function importData() {
        await importCategory.mutateAsync(importDefaultData);
    }

    async function run() {
        for (const category of initialData.categories) {
            await createCategory.mutateAsync({
                name:        category.name,
                description: category.description,
                root_path:   category.root_path,
            });

            const categoryResult = (await CategoryAPI.getAll())[0];

            for (const subject of category.subjects) {
                await createSubject.mutateAsync({
                    name:            subject.name,
                    description:     subject.description,
                    belong_category: categoryResult.id,
                });

                const subjectResult = (await SubjectAPI.query({ name: subject.name }))[0];

                for (const tag of subject.tags) {
                    tagMutation.mutateAsync({
                        name:            tag,
                        description:     `A description of ${tag}`,
                        belong_category: categoryResult.id,
                        belong_subject:  subjectResult.id,
                    });
                }
            }
        }
    }

    return (
        <Affix position={{ bottom: rem(20), left: rem(20) }}>
            <Button onClick={() => importData()}>Gen</Button>
        </Affix>
    );
}
