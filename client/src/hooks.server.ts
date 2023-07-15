export const handle = async ({ event, resolve }) => {
    const jwt = event.cookies.get('jwt');
    if (jwt) {
        (event.locals as any).user = jwt;
    }
    const response = await resolve(event);
    return response;
};