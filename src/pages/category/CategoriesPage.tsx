import { useState, useEffect } from 'react';
import { Grid, Skeleton } from '@mantine/core';
import { CategoryQuery } from '@api/category';
import { CategoryCard } from './components';

export function CategoriesPage() {
    const { data: categories, isLoading: isCategoriesLoading } = CategoryQuery.useGetAll();

    const categoryItems = categories.map((val) => <CategoryCard data={val} />);

    return (
        <Skeleton visible={isCategoriesLoading}>
            <Grid>
                {categoryItems}
            </Grid>
        </Skeleton>
    );
}
