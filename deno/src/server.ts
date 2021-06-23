import {Application} from "https://deno.land/x/oak/mod.ts";
import router from './routes.ts';

const port = 8000;

const app = new Application();

app.use(router.routes());

try {
	console.log(`Server listening on port ${port}`)
	await app.listen({port});
} catch (e) {
	console.log('err starting server:', e);
}