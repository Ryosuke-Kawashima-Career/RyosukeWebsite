// A Server Component (no "use client"): it has no state, effects or event handlers,
// so React renders it to HTML on the server and ships zero JavaScript for it.

import type { LinkedInPost } from "@/content/linkedin";
import { DEFAULT_EMBED_HEIGHT, embedUrl, postUrl, publishedAt } from "@/lib/linkedin";

type Props = { post: LinkedInPost };

export function LinkedInEmbed({ post }: Props) {
  const height = post.height ?? DEFAULT_EMBED_HEIGHT;
  const date = publishedAt(post.urn);

  return (
    <article className="mx-auto flex w-full max-w-[504px] flex-col gap-3 rounded-2xl border border-zinc-200 p-3 dark:border-zinc-800">
      <header className="flex items-baseline justify-between gap-2">
        <h2 className="text-base font-medium text-zinc-900 dark:text-zinc-100">
          {post.title}
        </h2>
        <time
          dateTime={date.toISOString()}
          className="shrink-0 text-xs text-zinc-500 dark:text-zinc-400"
        >
          {date.toLocaleDateString("en-US", {
            year: "numeric",
            month: "short",
            day: "numeric",
          })}
        </time>
      </header>

      {post.note && (
        <p className="text-sm text-zinc-600 dark:text-zinc-400">{post.note}</p>
      )}

      <iframe
        src={embedUrl(post.urn)}
        height={height}
        className="w-full rounded-lg"
        title={`LinkedIn post: ${post.title}`}
        loading="lazy"
        allowFullScreen
        sandbox="allow-scripts allow-same-origin allow-popups allow-popups-to-escape-sandbox"
        referrerPolicy="strict-origin-when-cross-origin"
      />

      <a
        href={postUrl(post.urn)}
        target="_blank"
        rel="noopener noreferrer"
        className="self-end text-sm text-zinc-600 underline underline-offset-4 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
      >
        View on LinkedIn ↗
      </a>
    </article>
  );
}
