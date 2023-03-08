import { createSlice } from '@reduxjs/toolkit';

const sliceFile = createSlice({
    name: 'file',
    initialState: {
        fid: undefined
    },
    reducers: {
        setFileId: (state, action) => {
            state.fid = action.payload;
        },
    },
});

export const { setFileId } = sliceFile.actions;
export const fileReducer = sliceFile.reducer;
