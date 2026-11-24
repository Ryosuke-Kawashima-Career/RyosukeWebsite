// Pure helper functions: input → output, no side effects.
// They are easy to reason about and to test, so keep logic here, not in components.

import { linkedinPosts, type LinkedInPost } from "@/content/linkedin";

export const DEFAULT_EMBED_HEIGHT = 628;

/**
 * Build the iframe URL from a LinkedIn URN.
 * embedUrl("urn:li:share:123")
 *   → "https://www.linkedin.com/embed/feed/update/urn:li:share:123"
 */
export function embedUrl(urn: string): string {
  return `https://www.linkedin.com/embed/feed/update/${urn}`;
}

/**
 * Build the normal (non-embed) post URL, used for the "View on LinkedIn" link.
 * postUrl("urn:li:share:123")
 *   → "https://www.linkedin.com/feed/update/urn:li:share:123"
 */
export function postUrl(urn: string): string {
  return `https://www.linkedin.com/feed/update/${urn}`;
}

/**
 * Recover the publish date from a LinkedIn URN.
 * LinkedIn IDs are 64-bit snowflake IDs whose top bits hold the creation time
 * in Unix epoch milliseconds (shifted left by 22 bits).
 */
export function publishedAt(urn: string): Date {
  const id = urn.split(":").at(-1);
  if (!id) {
    throw new Error(`Invalid LinkedIn URN: "${urn}"`);
  }
  const timestampMs = Number(BigInt(id) >> BigInt(22));
  return new Date(timestampMs);
}

/**
 * Return the posts sorted newest first without mutating the source array.
 */
export function postsNewestFirst(): LinkedInPost[] {
  return [...linkedinPosts].sort(
    (a, b) => publishedAt(b.urn).getTime() - publishedAt(a.urn).getTime(),
  );
}
