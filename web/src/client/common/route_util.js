export let ROUTE = {
    TEMPLATE: '/template',
    DRIVE: '/drive',
    SIGN_IN: '/signin',
    SIGN_UP: '/signup'
}

export function goto(router, route) {
    if (router.route != route) {
        router.push(route);
    }
}

export function redirect(router, route) {
    window.location.href = `${router.basePath}${ROUTE.TEMPLATE}`;
}