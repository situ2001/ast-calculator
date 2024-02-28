/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    useWasmBinary: true,
  },
  webpack: (configs, options) => {
    configs.experiments = {
      ...configs.experiments,
      asyncWebAssembly: true,
    };
    return configs;
  },
};

export default nextConfig;
