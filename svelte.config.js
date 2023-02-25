import staticAdapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [vitePreprocess()],
  kit: {
    alias: {
      service: "src/service",
    },
    adapter: staticAdapter(),
  },
};

export default config;
