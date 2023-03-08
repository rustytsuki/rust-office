import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import Card from 'react-bootstrap/Card';
import Nav from 'react-bootstrap/Nav';
import { useState, createRef } from 'react';
import { useRouter } from 'next/router';
import { ROUTE, goto } from '../common/route_util';
import { MessageBox } from '../common/MessageBox';

export function SignUp() {
    const router = useRouter();

    const [validated, setValidated] = useState(false);
    const [passwd1, setPassword1] = useState('');
    const [passwd2, setPassword2] = useState('');
    const [signup_success, set_signup_success] = useState(false);

    const formUser = createRef();
    const messageBox = createRef();

    let onSubmit = async (event) => {
        event.preventDefault();
        event.stopPropagation();

        if (!isPasswdOk()) {
            setValidated(true);
            return;
        }

        const form = event.currentTarget;
        if (form.checkValidity()) {
            console.log('haha');
            // post signup
            const user = formUser.current.value;
            const response = await fetch('/auth/signup', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ 'name': user.trim(), 'password': passwd1.trim() }),
            });

            const content = await response.json();
            if (content['success']) {
                messageBox.current.show("", "Sign up successfully! now please sign in.", false);
            } else {
                messageBox.current.show("", "Sign up error!", false);
            }
            set_signup_success(content['success']);
        }

        setValidated(true);
    };

    let onMsgConfirm = () => {
        if (signup_success) {
            goto(router, ROUTE.SIGN_IN);
        }
    }

    let onPasswd1Change = (event) => {
        const form = event.currentTarget;
        setPassword1(form.value);
    };

    let onPasswd2Change = (event) => {
        const form = event.currentTarget;
        setPassword2(form.value);
    };

    let isPasswordValid = () => {
        if (!validated) {
            return true;
        }

        return isPasswdOk();
    };

    let isPasswdOk = () => {
        if (passwd1.trim() === '' || passwd2.trim() === '') {
            return false;
        }

        if (passwd1 !== passwd2) {
            return false;
        }

        return true;
    }

    let getPasswordInvalidText = () => {
        if (passwd1.trim() === '' || passwd2.trim() === '') {
            return 'Password is a required field';
        }

        if (passwd1 !== passwd2) {
            return 'The passwords you typed do not match';
        }

        return '';
    };

    return (
        <>
            <Container>
                <Row>
                    <Col></Col>
                    <Col xs={6}>
                        <br />
                        <h2 style={{ 'textAlign': 'center' }}>Sign up to Rust-Office</h2>
                        <br />
                        <Card>
                            <Card.Body>
                                <Form noValidate validated={validated} onSubmit={onSubmit}>
                                    <Form.Group className="mb-3">
                                        <Form.Label>Username</Form.Label>
                                        <Form.Control
                                            ref={formUser}
                                            type="text"
                                            placeholder="Enter Username"
                                            required
                                        />
                                        <Form.Control.Feedback type="invalid">
                                            User name is a required field
                                        </Form.Control.Feedback>
                                    </Form.Group>

                                    <Form.Group className="mb-3">
                                        <Form.Label>Password</Form.Label>
                                        <Form.Control
                                            type="password"
                                            placeholder="Enter Password"
                                            required
                                            value={passwd1}
                                            onChange={onPasswd1Change}
                                            isInvalid={!isPasswordValid()}
                                        />
                                    </Form.Group>

                                    <Form.Group className="mb-3">
                                        <Form.Control
                                            type="password"
                                            placeholder="Enter Password again"
                                            required
                                            value={passwd2}
                                            onChange={onPasswd2Change}
                                            isInvalid={!isPasswordValid()}
                                        />
                                        <Form.Control.Feedback type="invalid">
                                            {getPasswordInvalidText()}
                                        </Form.Control.Feedback>
                                        <Form.Text className="text-muted">
                                            We'll never store and share your password with anyone else.
                                        </Form.Text>
                                    </Form.Group>

                                    <Button variant="outline-primary" type="submit">
                                        Sign up
                                    </Button>
                                </Form>
                            </Card.Body>
                        </Card>
                        <br />
                        <Card>
                            <Card.Body>
                                <Form.Label>Already have an account?</Form.Label>
                                <Button variant="link" onClick={() => { goto(router, ROUTE.SIGN_IN); }}>Sign in.</Button>
                            </Card.Body>
                        </Card>
                    </Col>
                    <Col></Col>
                </Row>
            </Container>

            <MessageBox ref={messageBox} onConfirm={onMsgConfirm} />
        </>
    );
}
