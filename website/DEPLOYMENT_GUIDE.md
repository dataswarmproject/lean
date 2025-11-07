# Deployment Guide

This guide explains how to deploy the LEAN Chain website to Vercel with dynamic URL support.

## Dynamic URL System

The website uses a dynamic URL system that automatically adapts to:
- **Localhost**: Uses path-based routing (e.g., `http://localhost:3000/docs`)
- **Vercel Preview**: Uses path-based routing (e.g., `https://your-app.vercel.app/docs`)
- **Production Domain**: Uses subdomain routing (e.g., `https://docs.leanchain.io`)

## Local Development

1. Install dependencies:
```bash
cd website
npm install
```

2. Run the development server:
```bash
npm run dev
```

3. Access the site at `http://localhost:3000`
   - Main page: `http://localhost:3000`
   - Docs: `http://localhost:3000/docs`
   - Forum: `http://localhost:3000/forum`
   - Validators: `http://localhost:3000/validators`
   - Explorer: `http://localhost:3000/explorer`
   - Staking: `http://localhost:3000/staking`
   - Status: `http://localhost:3000/status`

## Deploying to Vercel

### Initial Deployment (Random Domain)

1. **Install Vercel CLI** (if not already installed):
```bash
npm i -g vercel
```

2. **Deploy from the website directory**:
```bash
cd website
vercel
```

3. Follow the prompts:
   - Link to existing project or create new
   - Confirm project settings
   - Deploy

4. Vercel will provide a random domain like `your-app-xyz123.vercel.app`

5. The site will automatically use path-based routing on this domain:
   - Main: `https://your-app-xyz123.vercel.app`
   - Docs: `https://your-app-xyz123.vercel.app/docs`
   - etc.

### Switching to Custom Domain (leanchain.io)

1. **Add domain in Vercel Dashboard**:
   - Go to your project settings
   - Navigate to "Domains"
   - Add `leanchain.io` and `*.leanchain.io` (for subdomains)

2. **Configure DNS**:
   - Add A record pointing to Vercel's IP (provided in dashboard)
   - Add CNAME record for `*.leanchain.io` pointing to `cname.vercel-dns.com`

3. **Wait for DNS propagation** (usually 5-30 minutes)

4. **The site will automatically switch to subdomain routing**:
   - Main: `https://leanchain.io`
   - Docs: `https://docs.leanchain.io`
   - Forum: `https://forum.leanchain.io`
   - Validators: `https://validators.leanchain.io`
   - Explorer: `https://explorer.leanchain.io`
   - Staking: `https://staking.leanchain.io`
   - Status: `https://status.leanchain.io`

## Environment Variables

Optional environment variables (set in Vercel dashboard):

- `NEXT_PUBLIC_BASE_URL`: Override base URL (usually not needed, auto-detected)

## How It Works

The `getSubdomainUrl()` function in `lib/utils.ts`:
- Detects the current hostname
- For localhost/Vercel preview: returns path-based URLs
- For custom domains: returns subdomain-based URLs
- Automatically handles both client-side and server-side rendering

## Testing

1. **Test locally**: All URLs should work with path-based routing
2. **Test on Vercel preview**: URLs should work with path-based routing
3. **Test on production**: After DNS setup, URLs should work with subdomain routing

## Troubleshooting

- **URLs not working**: Check that middleware is properly routing subdomains
- **Subdomains not resolving**: Verify DNS configuration and wait for propagation
- **Path-based routing on production**: Check that domain is properly configured in Vercel

