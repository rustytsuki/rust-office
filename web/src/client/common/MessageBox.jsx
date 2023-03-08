import React from 'react';
import Modal from 'react-bootstrap/Modal';
import Button from 'react-bootstrap/Button';

export class MessageBox extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            title: '',
            desc: '',
            shown: false,
            showCancel: true,
        };
    }

    render() {
        return (
            <>
                <Modal show={this.state.shown} onHide={this.onCancel} backdrop="static" keyboard={false}>
                    <Modal.Header closeButton={this.state.showCancel}>
                        <Modal.Title>{this.state.title}</Modal.Title>
                    </Modal.Header>
                    <Modal.Body>
                        <p style={{ wordBreak: 'break-word' }}>{this.state.desc}</p>
                    </Modal.Body>
                    <Modal.Footer>
                        <Button variant="outline-primary" onClick={this.onConfirm}>
                            Ok
                        </Button>
                        {this.state.showCancel && (
                            <Button variant="outline-secondary" onClick={this.onCancel}>
                                Cancel
                            </Button>
                        )}
                    </Modal.Footer>
                </Modal>
            </>
        );
    }

    onCancel = () => {
        this.setState({
            shown: false,
        });

        this.props.onCancel && this.props.onCancel();
    };

    onConfirm = () => {
        this.setState({
            shown: false,
        });

        this.props.onConfirm && this.props.onConfirm();
    };

    show(title, desc, showCancel) {
        this.setState({
            title: title,
            desc: desc,
            shown: true,
            showCancel: !!showCancel,
        });
    }
}
