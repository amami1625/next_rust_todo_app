import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Turbopack の設定（Next.js 16 のデフォルトバンドラー）
  turbopack: {},
  // Webpack 用のポーリング設定（Turbopack が無効のときのフォールバック）
  // Docker 環境ではOSのファイル監視イベントが届かないためポーリングを使う
  webpack: (config) => {
    config.watchOptions = {
      poll: 1000,
      aggregateTimeout: 300,
    };
    return config;
  },
};

export default nextConfig;
