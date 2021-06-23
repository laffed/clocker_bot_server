import {Router} from "https://deno.land/x/oak/mod.ts";

const router = new Router();

router.get('/', context => {
	context.response.body = 'Hello World!';
});

export default router;