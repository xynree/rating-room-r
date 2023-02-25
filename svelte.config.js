import staticAdapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [vitePreprocess()],
	kit: {
		alias: {
			service: "src/service",
			store: "src/store.ts"
		},
		adapter: staticAdapter(),
		prerender: {
			entries: ["*", "/items/[id]", "/items/[id]/edit"],
		},
	},
};

export default config;
