import { TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux';
import { RootState, AppDispatch, store } from './store';

// Use slice specific hooks throughout your app instead of plain `useDispatch` and `useSelector`
const useAppDispatch = () => useDispatch<AppDispatch>();
const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;

// Global state
export const useGlobalSelector = () => useAppSelector((state = store.getState()) => state.global);
export const useGlobalDispatch = () => useAppDispatch();

// Model state
export const useModelSelector = () => useAppSelector((state = store.getState()) => state.modal);
export const useModelDispatch = () => useAppDispatch();
