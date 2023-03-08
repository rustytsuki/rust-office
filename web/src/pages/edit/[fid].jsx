import { useRouter } from "next/router";
import { useEffect } from "react";
import { Provider } from "react-redux";
import { Editor } from "../../client/edit/Editor";
import { store } from "../../client/edit/redux/store";
import { setFileId } from "../../client/edit/redux/sliceFile";

export default function EditPage() {
    const router = useRouter();
    useEffect(() => {
        if (!router.isReady) {
            return;
        }

        const { fid } = router.query;
        store.dispatch(setFileId(fid));

        return () => {
            store.dispatch(setFileId(undefined));
        }

    }, [router.isReady]);

    return (
        <Provider store={store}>
            <Editor />
        </Provider>
    );
}
