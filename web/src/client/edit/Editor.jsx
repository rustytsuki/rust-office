import { createRef } from 'react';
import styles from './Editor.module.scss';
import { Navigation } from './Navigation';
import { Canvas } from './Canvas';
import { MenuBar } from './MenuBar';

export function Editor() {
    return (
        <div id="editorContent" className={styles.root}>
            <div className={styles.menu}>
                <MenuBar />
            </div>
            <div className={styles.editor}>
                <div className={styles.navigation}>
                    <Navigation />
                </div>
                <div className={styles.canvas}>
                    <Canvas />
                </div>
            </div>
        </div>
    );
}
