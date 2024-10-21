import { LOGIN_SERVER_URL } from "./src/const/env.const";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react()],
	server: {
		proxy: {
			"/api": {
				target: LOGIN_SERVER_URL,
				changeOrigin: true,
				secure: false,
			},
		},
	},
});
