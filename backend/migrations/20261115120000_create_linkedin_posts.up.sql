-- LinkedIn posts curated by hand from LinkedIn's "Embed this post" feature.
-- See dev/plans/linkedin_curated_embeds_plan.md, section 4.2.
CREATE TABLE linkedin_posts (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    embed_url     TEXT NOT NULL UNIQUE,            -- validated, normalized by parse_embed
    post_url      TEXT,                            -- optional public link to the post
    urn           TEXT NOT NULL UNIQUE,            -- e.g. urn:li:share:123, used for dedupe
    note          TEXT,                            -- optional commentary
    height        INT NOT NULL DEFAULT 600 CHECK (height BETWEEN 200 AND 2000),
    featured      BOOLEAN NOT NULL DEFAULT FALSE,
    published_at  DATE,                            -- original post date, entered by hand
    position      INT,                             -- optional manual ordering
    hidden        BOOLEAN NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now()  -- set to now() by every UPDATE query
);

-- Serves the public feed: non-hidden posts, newest first.
CREATE INDEX linkedin_posts_feed_idx
    ON linkedin_posts (published_at DESC NULLS LAST, created_at DESC, id DESC)
    WHERE hidden = FALSE;
