/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack(config, { isServer }) {
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,

        fs: false, //"@pcd/gpc" relies on fs but we use some fns client side
      };
    }

    return config;
  },
};

export default nextConfig;
