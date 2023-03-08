import { configureStore } from '@reduxjs/toolkit';
import { engineReducer } from './sliceEngine';
import { fileReducer } from './sliceFile';

export const store = configureStore({
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware({
            serializableCheck: {
                ignoredActions: ['engine/setEngine'],
                ignoredPaths: ['engine.engine'],
            },
        }),

    reducer: {
        engine: engineReducer,
        fid: fileReducer,
    },
});
