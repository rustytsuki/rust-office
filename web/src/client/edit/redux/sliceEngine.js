import { createSlice } from '@reduxjs/toolkit';

const sliceEngine = createSlice({
    name: 'engine',
    initialState: {
        engine: undefined
    },
    reducers: {
        setEngine: (state, action) => {
            state.engine = action.payload;
        },
    },
});

export const { setEngine } = sliceEngine.actions;
export const engineReducer = sliceEngine.reducer;
