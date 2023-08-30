import { useCallback } from 'react';
import { useNavigate } from 'react-router-dom';

export function useCategoryNavigate() {
    const navigate = useNavigate();

    const jumpFn = useCallback((categoryId: string) => {
        navigate(`category/${categoryId}`, { replace: true });
    }, [navigate]);

    return jumpFn;
}
