// File-based routing: this file's path (app/linkedin/page.tsx) becomes the URL /linkedin.
// It uses no request data (cookies, headers, fetch), so Next.js prerenders it to static
// HTML at build time: fast, cheap, and easy for search engines to read.

import type { Metadata } from "next";
import { LinkedInEmbed } from "@/components/LinkedInEmbed";
import { postsNewestFirst } from "@/lib/linkedin";

export const metadata: Metadata = {
  title: "LinkedIn posts",
  description: "Selected posts from my LinkedIn.",
};

export default function LinkedInPage() {
  const posts = postsNewestFirst();

  return (
    <main className="mx-auto w-full max-w-5xl px-4 py-12">
      <h1 className="text-3xl font-semibold tracking-tight">LinkedIn posts</h1>
      <p className="mt-2 text-zinc-600 dark:text-zinc-400">
        Selected updates, projects, and milestones from my LinkedIn.
      </p>

      <div className="mt-8 grid gap-6 md:grid-cols-2">
        {posts.map((post) => (
          <LinkedInEmbed key={post.urn} post={post} />
        ))}
      </div>
    </main>
  );
}
