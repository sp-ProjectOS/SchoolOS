import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { internalIpV4 } from "internal-ip";
import tailwind from "tailwindcss"
import autoprefixer from "autoprefixer"
import path from "path";
import { templateCompilerOptions } from '@tresjs/core'

const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	css: {
		postcss: {
			plugins: [tailwind(), autoprefixer()],
		},
	},
	plugins: [vue({
		...templateCompilerOptions
	})],
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./src"),
		},
	},

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
		host: "0.0.0.0",
		hmr: mobile
			? {
				protocol: "ws",
				host: await internalIpV4(),
				port: 1421,
			}
			: undefined,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"],
		},
	},
}));
