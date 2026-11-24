// Content as code: this file is the "database" for the LinkedIn page.
// To add a post: LinkedIn → ⋯ → "Embed this post" → copy the URN from the src URL
// (the part "urn:li:ugcPost:123..." between "/update/" and "?") → add an entry → PR → merge.
//
// Only the URN is stored, never the raw <iframe> HTML. The URL is built in code
// (src/lib/linkedin.ts), so nothing pasted here can inject HTML into the site.

export type LinkedInPost = {
  /** e.g. "urn:li:ugcPost:7516897432014581760" */
  urn: string;
  /** Short label for you and for the iframe's accessible title. */
  title: string;
  /** Height in px from LinkedIn's snippet. Optional: defaults to 628. */
  height?: number;
  /** Your own commentary, shown above the embed. Optional. */
  note?: string;
};

export const linkedinPosts: LinkedInPost[] = [
  { urn: "urn:li:ugcPost:7516897432014581760", title: "IIT Delhi" },
  { urn: "urn:li:ugcPost:7492601560510717952", title: "Switchcode 3rd place" },
  { urn: "urn:li:ugcPost:7463277645934403584", title: "AI Dojo" },
  { urn: "urn:li:share:7448183212452933633", title: "Flight to India", height: 634 },
  { urn: "urn:li:ugcPost:7442750919345278976", title: "University Graduation" },
  { urn: "urn:li:ugcPost:7390005258573336577", title: "First flight to India" },
  { urn: "urn:li:ugcPost:7365392922126520320", title: "HPAIR" },
  { urn: "urn:li:ugcPost:7530310314869985280", title: "Muslim Wedding" },
  { urn: "urn:li:ugcPost:7528850996437897216", title: "First Hackathon Win with Cyberwave" },
  { urn: "urn:li:ugcPost:7523421193627086849", title: "DevFest at Google Office" },
  { urn: "urn:li:ugcPost:7527195653320466433", title: "Think Travel at Google Office" },
  { urn: "urn:li:ugcPost:7518258067789156352", title: "Flight to Singapore for ITB Asia 2026" },
];
