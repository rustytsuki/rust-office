import { useSelector } from 'react-redux';
// import styles from "./MenuBar.module.scss";
import Button from "react-bootstrap/Button";

export function MenuBar() {
    const { engine } = useSelector((state) => state.engine);

    let onOpenNavigator = () => {
        if (engine) {
        }
    };

    return (
        <div>
            <Button variant="primary" onClick={onOpenNavigator}>
                Navigation Pane
            </Button>
        </div>
    );
}
