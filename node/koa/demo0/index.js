const Koa = require('koa');
const Router = require('koa-router');
const static = require('koa-static');
const app = new Koa();
const router = new Router();
// logger

app.use(async (ctx, next) => {
  await next();
  const rt = ctx.response.get('X-Response-Time');
  console.log(`${ctx.method} ${ctx.url} - ${rt}`);
});

// x-response-time

app.use(async (ctx, next) => {
  const start = Date.now();
  await next();
  const ms = Date.now() - start;
  ctx.set('X-Response-Time', `${ms}ms`);
});

app.use(static(
  path.join(__dirname, 'static')
));

router.get('/', (ctx, next) => {
  ctx.body = 'get /'
});

router.post('/p', (ctx, next) => {
  ctx.body = {"res": "OK"}
});

app.use(router.routes())
  .use(router.allowedMethods());


app.listen(3000);