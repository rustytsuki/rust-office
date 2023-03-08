import { createRef, useEffect } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import styles from './Canvas.module.scss';
import { setEngine } from './redux/sliceEngine';

export function Canvas() {
    const { engine } = useSelector((state) => state.engine);
    const { fid } = useSelector((state) => state.fid);
    const domRef = createRef();
    const dispatch = useDispatch();
    let rust_office = null;

    let getFileTree = async function(id) {
        const response = await fetch("/drive/tree", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ "id": parseInt(id) }),
        });

        const content = await response.json();
        if (content["success"]) {
            let tree = content["payload"];
            return tree;
        }
    }

    // create engine
    useEffect(() => {
        if (!fid) {
            return;
        }

        let isSubscribed = true;

        let loadEngine = async (rootDom, fid) => {
            // await test(fid);
            let engine = await import('../../../../kernel/wasm32-unknown-unknown');
            await engine.default();
            if (isSubscribed) {
                rust_office = new engine.RustOffice(rootDom, fid);
                // dispatch(setEngine(rust_office));
                // let tree = await getFileTree(fid);
                // let ret = await rust_office.load(`/storage/${fid}/unpacked`, JSON.stringify(tree));
                // console.log("rust office loaded: " + ret);

                // egui
                rust_office.run();
            } else {
                rust_office = null;
            }
        };

        if (!rust_office) {
            loadEngine(domRef.current, fid).catch(console.error);
        }

        return () => {
            isSubscribed = false;

            if (rust_office) {
                dispatch(setEngine(undefined));
                rust_office.free();
                rust_office = null;
            }
        };
    }, [fid]); // the [] must be append, or the useEffect may called twice

    // register browser resize
    useEffect(() => {
        function onResize() {
            if (engine) {
                engine.resize();
            }
        }

        function onBeforeunload() {
            if (engine) {
                engine.free();
            }
        }

        // The *Down happens first, the *Press happens second (when text is entered), and the *Up happens last (when text input is complete)
        function onKeydown(event) {
            if (engine) {
                // engine.keydown()
            }
        }

        window.addEventListener('resize', onResize);
        window.addEventListener('beforeunload', onBeforeunload);
        window.addEventListener('keydown', onKeydown);

        return () => {
            window.removeEventListener('resize', onResize);
            window.removeEventListener('beforeunload', onBeforeunload);
            window.removeEventListener('keydown', onKeydown);
        };
    }, [engine]);

    return (
        <div id="edit-container" className={styles.root} ref={domRef} >
            <canvas id="rust-office"/>
        </div>
    );
}

async function test(fid) {
    let url = `/storage/${fid}/unpacked/sheet1.xml`;
    url = `/storage/${fid}/unpacked/_rels/.rels`;
    
    // let xhr = new XMLHttpRequest();
    // xhr.overrideMimeType('application/xml');
    // xhr.open("GET", url, true);
    // console.time("load");
    // xhr.onload = () => {
    //     console.timeEnd("load");
    //     console.time("parse");
    //     let doc = xhr.responseXML;
    //     console.timeEnd("parse");
    //     // let root = doc.firstElementChild;
    //     // console.log(root.localName);
    // }
    // xhr.send();

    // console.time("fetch");
    // let resp = await fetch(url);
    // let xml_str = await resp.text();
    // console.timeEnd("fetch");
    // let parser = new DOMParser();
    // console.time("parse");
    // let doc = parser.parseFromString(xml_str, 'application/xml');
    // console.timeEnd("parse");
    // let root = doc.firstElementChild;
    // console.log(root.localName);

    let xml_str = `
    <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
        <w:body>
            <w:sectPr w:rsidR="00EA62F7">
                <w:pgSz w:w="11906" w:h="16838"/>
                <w:pgMar w:top="1440" w:right="1800" w:bottom="1440" w:left="1800" w:header="851" w:footer="992" w:gutter="0"/>
                <w:cols w:space="425"/>
                <w:docGrid w:type="lines" w:linePitch="312"/>
            </w:sectPr>
        </w:body>
    </w:document>
    `;
    let parser = new DOMParser();
    let doc = parser.parseFromString(xml_str, 'application/xml');
    let root = doc.firstElementChild;
    let body = root.firstElementChild;
    let sectPr = body.firstElementChild;
    let pgSz = sectPr.firstElementChild;
    let attr = pgSz.attributes;
    console.log(pgSz.localName);
}