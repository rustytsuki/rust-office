import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import Card from 'react-bootstrap/Card';
import Nav from 'react-bootstrap/Nav';
import { useState, createRef } from 'react';
import { useRouter } from "next/router";
import { ROUTE, goto, redirect } from '../common/route_util';
import { MessageBox } from '../common/MessageBox';

export function SignIn() {
    const router = useRouter();

    const [validated, setValidated] = useState(false);
    const formUser = createRef();
    const formPassword = createRef();
    const messageBox = createRef();

    let onSubmit = async (event) => {
        event.preventDefault();
        event.stopPropagation();

        const form = event.currentTarget;
        if (form.checkValidity()) {
            // post signin
            const user = formUser.current.value;
            const password = formPassword.current.value;

            const response = await fetch('/auth/signin', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ 'name': user.trim(), 'password': password.trim() }),
            });

            const content = await response.json();
            if (content['success']) {
                redirect(router, ROUTE.TEMPLATE);
            } else {
                messageBox.current.show("", "Sign in error!", false);
            }
        }

        setValidated(true);
    };

    return (
        <>
            <Container>
                <Row>
                    <Col></Col>
                    <Col xs={6}>
                        <br />
                        <h2 style={{ 'textAlign': 'center' }}>Sign in to Rust-Office</h2>
                        <br />
                        <Card>
                            <Card.Body>
                                <Form noValidate validated={validated} onSubmit={onSubmit}>
                                    <Form.Group className="mb-3">
                                        <Form.Label>Username</Form.Label>
                                        <Form.Control ref={formUser} type="text" required placeholder="Enter Username" />
                                    </Form.Group>

                                    <Form.Group className="mb-3">
                                        <Form.Label>Password</Form.Label>
                                        <Form.Control ref={formPassword} type="password" required placeholder="Enter Password" />
                                    </Form.Group>
                                    <Button variant="outline-primary" type="submit">
                                        Sign in
                                    </Button>
                                </Form>
                            </Card.Body>
                        </Card>
                        <br />
                        <Card>
                            <Card.Body>
                                <Form.Label>New to Rust-Office?</Form.Label>
                                <Button variant="link" onClick={() => { goto(router, ROUTE.SIGN_UP); }}>Create an account.</Button>
                            </Card.Body>
                        </Card>
                    </Col>
                    <Col></Col>
                </Row>
            </Container>

            <MessageBox ref={messageBox} />
        </>
    );
}
