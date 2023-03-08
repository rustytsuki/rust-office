import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import Nav from 'react-bootstrap/Nav';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import Dropdown from 'react-bootstrap/Dropdown';
import DropdownButton from 'react-bootstrap/DropdownButton';
import { useState, useEffect } from 'react';
import { useRouter } from 'next/router';
import { ROUTE, goto, redirect } from './route_util';

export function Navigator() {
    const router = useRouter();

    const [loaded, setLoaded] = useState(false);
    const [user, setUserName] = useState(null);

    useEffect(() => {
        if (window.session && window.session.user) {
            setUserName({
                id: window.session.user.id,
                name: window.session.user.name
            });
        }

        setLoaded(true);

        return () => {
            setUserName(null);
        };
    }, []);

    let onSignout = async () => {
        const response = await fetch('/auth/signout', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({}),
        });

        const content = await response.json();
        if (content['success']) {
            redirect(router, ROUTE.TEMPLATE);
        }
    };

    return (
        <Navbar bg="light" expand="sm" sticky="top">
            <Container>
                <Navbar.Brand>Rust-Office</Navbar.Brand>
                <Navbar.Toggle aria-controls="basic-navbar-nav" />
                <Navbar.Collapse id="basic-navbar-nav">
                    <Nav className="me-auto">
                        <Nav.Link
                            active={router.route === ROUTE.TEMPLATE}
                            onClick={() => {
                                goto(router, ROUTE.TEMPLATE);
                            }}
                        >
                            Templates
                        </Nav.Link>
                        {loaded && user && (
                            <Nav.Link
                                active={router.route === ROUTE.DRIVE}
                                onClick={() => {
                                    goto(router, ROUTE.DRIVE);
                                }}
                            >
                                My Files
                            </Nav.Link>
                        )}
                    </Nav>
                    {loaded && !user && (
                        <Form className="d-flex">
                            <Button
                                variant="link"
                                onClick={() => {
                                    goto(router, ROUTE.SIGN_IN);
                                }}
                            >
                                Sign In
                            </Button>
                            <Button
                                variant="outline-success"
                                onClick={() => {
                                    goto(router, ROUTE.SIGN_UP);
                                }}
                            >
                                Sign Up
                            </Button>
                        </Form>
                    )}
                    {loaded && user && (
                        <Form className="d-flex">
                            <DropdownButton variant="outline-success" title={user.name} menuAlign="right">
                                <Dropdown.Item onClick={onSignout}>Sign out</Dropdown.Item>
                            </DropdownButton>
                        </Form>
                    )}
                </Navbar.Collapse>
            </Container>
        </Navbar>
    );
}
