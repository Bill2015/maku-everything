import { useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import { ResourceAPI } from './ResourceAPI';
import { ResourceTagDto } from './Dto';

export interface IResourceTagGroup {
    subjectName: string;

    subjectId: string;

    tags: ResourceTagDto[];
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace ResourceQuery {
    export function useGetAll() {
        const queryfn = () => ResourceAPI.getAll();

        return useQuery(
            ['resource'],
            queryfn,
            {
                placeholderData: [],
                initialData:     [],
            },
        );
    }

    export function useGetById(id: string) {
        const queryfn = () => ResourceAPI.getById(id);

        return useQuery(
            ['resurce', id],
            queryfn,
            {
                placeholderData: null,
                initialData:     null,
            },
        );
    }

    export function useGetDetail(id: string) {
        const queryfn = () => ResourceAPI.getDetail(id);

        const { data: resourceData, ...query } = useQuery(
            ['resurce-detail', id],
            queryfn,
            {
                placeholderData: null,
                initialData:     null,
            },
        );

        // Mapping the tag by subjectName
        const resourceTagData: IResourceTagGroup[] = useMemo(() => {
            const map: Map<string, IResourceTagGroup> = new Map();

            if (resourceData) {
                for (const obj of resourceData.tags) {
                    // Initial the map value
                    if (map.has(obj.subject_name) === false) {
                        map.set(obj.subject_name, {
                            subjectId:   obj.belong_subject,
                            subjectName: obj.subject_name,
                            tags:        [],
                        });
                    }
                    map.get(obj.subject_name)!.tags.push(obj);
                }
            }

            return Array.from(map.values());
        }, [resourceData]);

        const subjects: Set<string> = useMemo(() => {
            if (resourceData) {
                return new Set<string>(resourceData.tags.map((val) => val.belong_subject));
            }
            return new Set();
        }, [resourceData]);

        return {
            data:       resourceData,
            tagMapData: resourceTagData,
            subjects:   subjects,
            ...query,
        };
    }
}
