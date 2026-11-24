import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Produces a minimal self-contained server in .next/standalone (used by the Docker image).
  output: "standalone",
  cacheComponents: true,
  partialPrefetching: true,
  // Restrict iframes to LinkedIn only via Content-Security-Policy.
  async headers() {
    return [
      {
        source: "/:path*",
        headers: [
          {
            key: "Content-Security-Policy",
            value: "frame-src https://www.linkedin.com",
          },
        ],
      },
    ];
  },
  turbopack: {
    rules: {
      "*.css": {
        loaders: ["@tailwindcss/turbopack"],
        as: "*.css",
      },
    },
  },
};

export default nextConfig;
