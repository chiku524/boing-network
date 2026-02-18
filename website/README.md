# boing.network Website

Static site for [boing.network](https://boing.network) — built with Astro, deployed to Cloudflare Pages.

## Setup

```bash
npm install
npm run dev
```

## Build

```bash
npm run build
```

Output: `dist/`

## Deploy to Cloudflare Pages

1. Connect the repo in Cloudflare dashboard (Workers & Pages → Create → Pages → Connect to Git)
2. Build command: `cd website && npm run build`
3. Build output: `website/dist`
4. Add custom domain: `boing.network`

Or via Wrangler:

```bash
cd website
npm run build
wrangler pages deploy dist --project-name=boing-network
```

## Cloudflare Resources

- **WEBSITE-SPEC.md** — Site structure and content mapping
- **CLOUDFLARE-SETUP.md** — Step-by-step D1, R2, KV setup
- **schema.sql** — D1 schema for block explorer / network stats
