import { useCallback } from 'react';
import { useNavigate } from 'react-router-dom';

export function useCategoryNavigate() {
    const navigate = useNavigate();

    const jumpFn = useCallback((categoryName: string) => {
        navigate(`category/${categoryName}`);
    }, [navigate]);

    return jumpFn;
}

export function useResourceDetailNavigate() {
    const navigate = useNavigate();

    const jumpFn = useCallback((belongCategoryName: string | null, resourceId: string | null) => {
        if (belongCategoryName && resourceId) {
            navigate(`resource/${resourceId}`);
        }
    }, [navigate]);

    return jumpFn;
}
