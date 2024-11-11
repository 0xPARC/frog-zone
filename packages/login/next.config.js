/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack(config, { isServer }) {
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,

        fs: false, //"@pcd/gpc" relies on fs but we use some fns client side
      };
      config.resolve.alias = {
        constants: require.resolve(
          "rollup-plugin-node-polyfills/polyfills/constants"
        ),
        process: "process/browser"
      };
    }

    return config;
  },
  experimental: {
    outputFileTracingRoot: path.join(__dirname, "..", ".."),
    outputFileTracing: true,
    outputFileTracingIncludes: {
      "/login": "node_modules/@pcd/proto-pod-gpc-artifacts",
    }
  }
};

module.exports = nextConfig;
