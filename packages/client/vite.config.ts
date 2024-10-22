import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
	const env = loadEnv(mode, process.cwd());
	return {
		plugins: [react()],
		server: {
			proxy: {
				"/api": {
					target: env.VITE_LOGIN_SERVER_URL,
					changeOrigin: true,
					secure: false,
				},
			},
		},
	};
});
