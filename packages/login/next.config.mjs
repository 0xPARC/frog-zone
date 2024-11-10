import CopyPlugin from "copy-webpack-plugin";

/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack(config, { isServer }) {
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,

        fs: false, //"@pcd/gpc" relies on fs but we use some fns client side
      };
    }

    const artifactPackageJsonPath = require.resolve('@pcd/proto-pod-gpc-artifacts/package.json');
    const artifactPath = path.dirname(artifactPackageJsonPath);
    config.plugins.push(
      new CopyPlugin({
        patterns: [
          { 
            from: artifactPath, 
            to: path.join(__dirname, 'public/artifacts'),
            force: true
          }
        ]
      })
    );

    return config;
  },
};

export default nextConfig;
