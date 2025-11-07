# Deployment Guide

This guide covers deploying the LEAN Chain website and all subdomains.

## Prerequisites

- Node.js 18+ installed
- Domain name (leanchain.io) configured
- Server with root/sudo access (for self-hosting)
- Or Vercel/Netlify account (for platform hosting)

## Option 1: Vercel Deployment (Recommended)

Vercel is the easiest option and handles all subdomain routing automatically.

### Steps

1. **Push code to GitHub**
   ```bash
   git add .
   git commit -m "Add website"
   git push origin main
   ```

2. **Import to Vercel**
   - Go to [vercel.com](https://vercel.com)
   - Click "New Project"
   - Import your GitHub repository
   - Set root directory to `website`
   - Click "Deploy"

3. **Configure Domain**
   - Go to Project Settings > Domains
   - Add `leanchain.io` and `www.leanchain.io`
   - Add all subdomains:
     - `forum.leanchain.io`
     - `docs.leanchain.io`
     - `validators.leanchain.io`
     - `explorer.leanchain.io`
     - `staking.leanchain.io`
     - `status.leanchain.io`

4. **Configure DNS**
   Add CNAME records pointing to Vercel:
   ```
   CNAME  leanchain.io          -> cname.vercel-dns.com
   CNAME  www.leanchain.io      -> cname.vercel-dns.com
   CNAME  forum.leanchain.io    -> cname.vercel-dns.com
   CNAME  docs.leanchain.io     -> cname.vercel-dns.com
   CNAME  validators.leanchain.io -> cname.vercel-dns.com
   CNAME  explorer.leanchain.io -> cname.vercel-dns.com
   CNAME  staking.leanchain.io  -> cname.vercel-dns.com
   CNAME  status.leanchain.io   -> cname.vercel-dns.com
   ```

5. **Configure Rewrites** (in `next.config.js`)
   Vercel will automatically handle subdomain routing based on the Host header.

## Option 2: Self-Hosted with Nginx

### Server Setup

1. **Install Node.js**
   ```bash
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs
   ```

2. **Clone and Build**
   ```bash
   cd /var/www
   git clone https://github.com/dataswarmproject/lean.git
   cd lean/website
   npm install
   npm run build
   ```

3. **Install PM2**
   ```bash
   npm install -g pm2
   pm2 start npm --name "leanchain-website" -- start
   pm2 save
   pm2 startup
   ```

4. **Install Nginx**
   ```bash
   sudo apt-get update
   sudo apt-get install nginx
   ```

5. **Configure Nginx**
   ```bash
   sudo cp deploy/nginx.conf /etc/nginx/sites-available/leanchain
   sudo ln -s /etc/nginx/sites-available/leanchain /etc/nginx/sites-enabled/
   sudo nginx -t
   sudo systemctl reload nginx
   ```

6. **Install SSL Certificate**
   ```bash
   sudo apt-get install certbot python3-certbot-nginx
   sudo certbot --nginx -d leanchain.io -d www.leanchain.io \
     -d forum.leanchain.io -d docs.leanchain.io \
     -d validators.leanchain.io -d explorer.leanchain.io \
     -d staking.leanchain.io -d status.leanchain.io
   ```

7. **Configure DNS**
   Point all A records to your server IP:
   ```
   A     leanchain.io            -> YOUR_SERVER_IP
   A     www.leanchain.io        -> YOUR_SERVER_IP
   A     forum.leanchain.io      -> YOUR_SERVER_IP
   A     docs.leanchain.io       -> YOUR_SERVER_IP
   A     validators.leanchain.io -> YOUR_SERVER_IP
   A     explorer.leanchain.io   -> YOUR_SERVER_IP
   A     staking.leanchain.io    -> YOUR_SERVER_IP
   A     status.leanchain.io     -> YOUR_SERVER_IP
   ```

## Option 3: Docker Deployment

1. **Build Docker Image**
   ```bash
   cd website
   docker build -t leanchain-website .
   ```

2. **Run with Docker Compose**
   ```bash
   cd deploy
   docker-compose up -d
   ```

3. **Configure Nginx** (same as Option 2)

## Environment Variables

Create `.env.production`:

```env
NEXT_PUBLIC_API_URL=https://api.leanchain.io
NEXT_PUBLIC_RPC_URL=https://rpc.leanchain.io
NEXT_PUBLIC_EXPLORER_URL=https://explorer.leanchain.io
```

## Monitoring

### Health Check

The website includes a health check endpoint:
- URL: `https://leanchain.io/api/health`
- Returns: JSON with status, timestamp, service name, and version

### PM2 Monitoring

```bash
pm2 status
pm2 logs leanchain-website
pm2 monit
```

## Troubleshooting

### Build Errors

- Ensure Node.js 18+ is installed
- Clear `.next` directory and rebuild
- Check for TypeScript errors: `npm run lint`

### Nginx Errors

- Test configuration: `sudo nginx -t`
- Check logs: `sudo tail -f /var/log/nginx/error.log`
- Verify DNS resolution: `dig leanchain.io`

### SSL Certificate Issues

- Renew certificate: `sudo certbot renew`
- Check expiration: `sudo certbot certificates`

## Performance Optimization

1. **Enable Caching**
   - Configure CDN (Cloudflare, etc.)
   - Enable Next.js static generation where possible

2. **Monitor Performance**
   - Use Vercel Analytics (if on Vercel)
   - Set up monitoring (UptimeRobot, etc.)

3. **Optimize Images**
   - Use Next.js Image component
   - Compress images before upload

## Security

1. **Keep Dependencies Updated**
   ```bash
   npm audit
   npm update
   ```

2. **Enable Security Headers**
   Already configured in `next.config.js`

3. **Regular Backups**
   - Backup code repository
   - Backup environment variables

## Support

For deployment issues, check:
- [Next.js Deployment Docs](https://nextjs.org/docs/deployment)
- [Vercel Documentation](https://vercel.com/docs)
- [Nginx Documentation](https://nginx.org/en/docs/)

