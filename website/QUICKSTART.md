# Quick Start Guide

Get the LEAN Chain website up and running in minutes.

## Local Development

```bash
# Navigate to website directory
cd website

# Install dependencies
npm install

# Run development server
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

## Available Routes

- `/` - Main landing page (leanchain.io)
- `/forum` - Forum (forum.leanchain.io)
- `/docs` - Documentation (docs.leanchain.io)
- `/validators` - Validator Dashboard (validators.leanchain.io)
- `/explorer` - Block Explorer (explorer.leanchain.io)
- `/staking` - Staking Calculator (staking.leanchain.io)
- `/status` - Network Status (status.leanchain.io)

## Testing Subdomains Locally

To test subdomain routing locally, edit your `/etc/hosts` file (Linux/Mac) or `C:\Windows\System32\drivers\etc\hosts` (Windows):

```
127.0.0.1 leanchain.io
127.0.0.1 forum.leanchain.io
127.0.0.1 docs.leanchain.io
127.0.0.1 validators.leanchain.io
127.0.0.1 explorer.leanchain.io
127.0.0.1 staking.leanchain.io
127.0.0.1 status.leanchain.io
```

Then access:
- http://leanchain.io:3000
- http://forum.leanchain.io:3000
- etc.

## Build for Production

```bash
npm run build
npm start
```

## Deploy

See [DEPLOYMENT.md](./DEPLOYMENT.md) for detailed deployment instructions.

## Project Structure

```
website/
├── app/                    # Next.js app directory
│   ├── page.tsx           # Main landing page
│   ├── forum/             # Forum subdomain
│   ├── docs/              # Documentation subdomain
│   ├── validators/        # Validator dashboard
│   ├── explorer/          # Block explorer
│   ├── staking/           # Staking calculator
│   ├── status/            # Network status
│   └── api/               # API routes
├── deploy/                # Deployment configs
│   ├── nginx.conf         # Nginx configuration
│   └── docker-compose.yml # Docker setup
├── middleware.ts          # Subdomain routing
├── package.json
└── README.md
```

## Features

✅ Modern, responsive design  
✅ Subdomain routing support  
✅ Interactive charts and visualizations  
✅ Real-time data displays  
✅ SEO optimized  
✅ Performance optimized  
✅ Security headers configured  

## Need Help?

- Check [README.md](./README.md) for detailed documentation
- See [DEPLOYMENT.md](./DEPLOYMENT.md) for deployment help
- Visit [docs.leanchain.io](https://docs.leanchain.io) for project docs

