import React from "react";
import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Button from "react-bootstrap/Button";
import ListGroup from "react-bootstrap/ListGroup";
import Form from "react-bootstrap/Form";
import Modal from "react-bootstrap/Modal";
import { useRouter } from "next/router";
import { MessageBox } from '../common/MessageBox';
import { Navigator } from '../common/Navigator';

class _Drive extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            activeKey: 0,
            files: [],
            showConfirm: false,
        };
        this._fileInputRef = React.createRef();
        this._fileSubmitRef = React.createRef();
        this._messageBox = React.createRef();
    }

    render() {
        const fileList = this.state.files.map((file, i) => {
            return (
                <ListGroup.Item key={i} eventKey={i} action>
                    {file.id}
                </ListGroup.Item>
            );
        });

        return (
            <>
                <Navigator />
                <Container style={{ marginTop: "20px" }}>
                    <Row>
                        <Col md={10}>
                            <Form id="file_1" ref={this._fileSubmitRef} style={{ display: "none" }} encType="multipart/form-data" onChange={this.onSubmitUpload}>
                                <Form.Control ref={this._fileInputRef} type="file" name="file" accept=".docx,.xlsx,.pptx" />
                            </Form>
                        </Col>
                    </Row>
                    <Row>
                        <Col md={10}>
                            <div style={{ margin: "10px", width: "100%" }}>
                                <Button variant="outline-primary" onClick={this.onBtNewClick}>
                                    New
                                </Button>{" "}
                                <Button variant="outline-primary" onClick={this.onBtUploadClick}>
                                    Upload
                                </Button>{" "}
                                <Button variant="outline-success" onClick={this.onBtEditClick}>
                                    Edit
                                </Button>{" "}
                                <Button variant="outline-danger" onClick={this.onBtDeleteClick}>
                                    Delete
                                </Button>{" "}
                                <Button variant="outline-primary" onClick={this.onBtOfdClick}>
                                    Open Native Folder
                                </Button>
                            </div>
                        </Col>
                    </Row>
                    <Row>
                        <Col md={10}>
                            <ListGroup
                                style={{ margin: "10px", width: "100%" }}
                                variant="flush"
                                activeKey={this.state.activeKey}
                                defaultActiveKey={0}
                                onSelect={this.onFileSelect}
                            >
                                {fileList}
                            </ListGroup>
                        </Col>
                    </Row>
                </Container>

                <Modal show={this.state.showConfirm} onHide={this.onCancel}>
                    <Modal.Header closeButton>
                        <Modal.Title>Warning</Modal.Title>
                    </Modal.Header>
                    <Modal.Body>Do you really want to delete this file?</Modal.Body>
                    <Modal.Footer>
                        <Button variant="outline-secondary" onClick={this.onCancel}>
                            Cancel
                        </Button>
                        <Button variant="outline-primary" onClick={this.onConfirm}>
                            Ok
                        </Button>
                    </Modal.Footer>
                </Modal>

                <MessageBox ref={this._messageBox} />
            </>
        );
    }

    onBtNewClick = async () => {
        let file = await this.createNewFile();

        this.setState({
            files: [file, ...this.state.files],
        });
    };

    onBtUploadClick = async () => {
        this._fileInputRef.current.click();
    };

    onSubmitUpload = async () => {
        const data = new FormData(this._fileSubmitRef.current);
        console.log(JSON.stringify(data));
        this._fileSubmitRef.current.reset();
        const response = await fetch("/drive/upload", {
            method: "post",
            body: data,
        });
        const content = await response.json();
        if (content["success"]) {
            let file = content["payload"];
            this.setState({
                files: [file, ...this.state.files],
            });
        }
    };

    onBtEditClick = () => {
        const id = this.getActivedFileId();
        if (!id) {
            return;
        }
        this.editFile(id);
    };

    onBtDeleteClick = async () => {
        const id = this.getActivedFileId();
        if (!id) {
            return;
        }
        this.setState({
            showConfirm: true,
        });
    };

    onBtOfdClick = async () => {
        const id = this.getActivedFileId();
        if (!id) {
            return;
        }
        const response = await fetch("/drive/ofd", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ "id": id }),
        });

        const content = await response.json();
        if (content["success"]) {
        }
    };

    onFileSelect = (activeKey) => {
        this.setState({
            activeKey: activeKey,
        });
    };

    onConfirm = async () => {
        const id = this.getActivedFileId();
        if (!id) {
            return;
        }
        this.setState({
            showConfirm: false,
        });
        await this.removeFile(id);
    };

    onCancel = () => {
        this.setState({
            showConfirm: false,
        });
    };

    async componentDidMount() {
        try {
            let files = await this.loadFiles();
            this.setState({
                files: [...files],
            });
        } catch (e) {
            console.log(e.message);
        }
    }

    getActivedFileId() {
        const file = this.getActivedFile();
        if (!file) {
            return null;
        }

        return file.id;
    }

    getActivedFile() {
        if (this.state.files.length === 0) {
            return null;
        }

        return this.state.files[this.state.activeKey];
    }

    async loadFiles() {
        const response = await fetch(`/drive/files`);
        const content = await response.json();
        if (content["success"]) {
            let files = content["payload"];
            return files;
        } else {
            return [];
        }
    }

    async createNewFile() {
        const response = await fetch("/drive/new", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                "title": "New File"
            }),
        });

        const content = await response.json();
        if (content["success"]) {
            let file = content["payload"];
            return file;
        }
    }

    editFile(id) {
        this.props.router.push(`/edit/${id}`);
    }

    async removeFile(id) {
        const response = await fetch("/drive/delete", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ "id": id }),
        });

        const content = await response.json();
        if (content["success"]) {
            const fid = content["payload"];
            let files = [];
            this.state.files.forEach((file) => {
                if (file.id !== fid) {
                    files.push(file);
                }
            });
            this.setState({ files });
        }
    }
}

export const Drive = React.forwardRef((props, ref) => {
    const router = useRouter();
    return <_Drive {...props} ref={ref} router={router} />;
});
