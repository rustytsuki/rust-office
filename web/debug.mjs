import next from "next";
import Koa from "koa";
import koaRouter from "koa-router";
import koaBody from "koa-body";
import bodyParser from "koa-bodyparser";

// must use renderToHTML to get rid of reverse-proxy issue
function page(router, nextApp) {
    const handleNext = nextApp.getRequestHandler();

    async function handleHtml(ctx, pathName) {
        let html = await nextApp.renderToHTML(ctx.req, ctx.res, pathName);

        ctx.type = "html";
        ctx.body = html;
    }

    router.get("/rusty/signin", async (ctx) => {
        await handleHtml(ctx, "/signin");
    });

    router.get("/rusty/signup", async (ctx) => {
        await handleHtml(ctx, "/signup");
    });

    router.get("/rusty/template", async (ctx) => {
        await handleHtml(ctx, "/template");
    });

    router.get("/rusty/drive", async (ctx) => {
        await handleHtml(ctx, "/drive");
    });

    router.get("/rusty/edit/:fid", async (ctx) => {
        const fid = ctx.params.fid;
        await handleHtml(ctx, `/edit/${fid}`);
    });

    router.get('(.*)', async ctx => {
        await handleNext(ctx.req, ctx.res);
        ctx.respond = false;
    });
}

async function main() {
    const isDebug = process.env.NODE_ENV !== "production";
    const nextApp = next({ dev: isDebug });

    await nextApp.prepare();
    
    const server = new Koa();
    server.use(
        koaBody({
            multipart: true,
            formidable: {
                maxFieldsSize: 100 * 1024 * 1024,
                multipart: true,
            },
        })
    );
    server.use(bodyParser());

    // set router
    const router = koaRouter();
    server.use(router.routes());
    server.use(router.allowedMethods());

    // page
    page(router, nextApp);

    server.listen(3048);
}

main();