export type InitialDataType = {
    categories: {
        name: string;
        description: string;
        root_path: string;
        subjects: {
            name: string;
            description: string;
            tags: string[];
        }[]
    }[]
}
