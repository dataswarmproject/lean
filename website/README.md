# LEAN Chain Website

Official website and subdomains for LEAN Chain blockchain platform.

## Subdomains

- **Main Site**: [leanchain.io](https://leanchain.io)
- **Forum**: [forum.leanchain.io](https://forum.leanchain.io)
- **Documentation**: [docs.leanchain.io](https://docs.leanchain.io)
- **Validator Dashboard**: [validators.leanchain.io](https://validators.leanchain.io)
- **Block Explorer**: [explorer.leanchain.io](https://explorer.leanchain.io)
- **Staking Calculator**: [staking.leanchain.io](https://staking.leanchain.io)
- **Network Status**: [status.leanchain.io](https://status.leanchain.io)

## Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **Charts**: Recharts
- **Icons**: Heroicons

## Getting Started

### Prerequisites

- Node.js 18+ 
- npm or yarn

### Installation

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

The development server will start at [http://localhost:3000](http://localhost:3000)

## Project Structure

```
website/
├── app/
│   ├── page.tsx              # Main landing page (leanchain.io)
│   ├── forum/
│   │   └── page.tsx          # Forum subdomain
│   ├── docs/
│   │   └── page.tsx          # Documentation subdomain
│   ├── validators/
│   │   └── page.tsx          # Validator dashboard
│   ├── explorer/
│   │   └── page.tsx          # Block explorer
│   ├── staking/
│   │   └── page.tsx          # Staking calculator
│   ├── status/
│   │   └── page.tsx          # Network status
│   ├── layout.tsx             # Root layout
│   └── globals.css            # Global styles
├── public/                    # Static assets
├── package.json
├── next.config.js
├── tailwind.config.js
└── tsconfig.json
```

## Dynamic URL System

The website automatically adapts URLs based on the deployment environment:

- **Localhost**: Uses path-based routing (`http://localhost:3000/docs`)
- **Vercel Preview**: Uses path-based routing (`https://your-app.vercel.app/docs`)
- **Production Domain**: Uses subdomain routing (`https://docs.leanchain.io`)

All URLs are generated dynamically using the `getSubdomainUrl()` utility function. No configuration needed!

## Deployment

### Vercel (Recommended)

See [DEPLOYMENT_GUIDE.md](./DEPLOYMENT_GUIDE.md) for detailed instructions.

**Quick Start:**

1. Push code to GitHub
2. Import project in Vercel
3. Deploy (Vercel will provide a random domain)
4. Add custom domain `leanchain.io` in Vercel dashboard
5. Configure DNS records

The site will automatically:
- Use path-based routing on preview/random domains
- Switch to subdomain routing when custom domain is configured
- Handle all URL generation dynamically

Vercel will automatically handle:
- SSL certificates
- Subdomain routing
- CDN distribution
- Automatic deployments

### Manual Deployment

1. Build the project:
   ```bash
   npm run build
   ```

2. Start the production server:
   ```bash
   npm start
   ```

3. Configure reverse proxy (nginx example):
   ```nginx
   server {
       server_name leanchain.io www.leanchain.io;
       location / {
           proxy_pass http://localhost:3000;
       }
   }
   
   server {
       server_name forum.leanchain.io;
       location / {
           proxy_pass http://localhost:3000/forum;
       }
   }
   
   # Similar configuration for other subdomains
   ```

### Subdomain Configuration

Configure DNS records to point subdomains to your server:

```
A     leanchain.io            -> YOUR_SERVER_IP
A     forum.leanchain.io      -> YOUR_SERVER_IP
A     docs.leanchain.io       -> YOUR_SERVER_IP
A     validators.leanchain.io -> YOUR_SERVER_IP
A     explorer.leanchain.io   -> YOUR_SERVER_IP
A     staking.leanchain.io    -> YOUR_SERVER_IP
A     status.leanchain.io     -> YOUR_SERVER_IP
```

## Environment Variables

Create a `.env.local` file for local development:

```env
NEXT_PUBLIC_API_URL=https://api.leanchain.io
NEXT_PUBLIC_RPC_URL=https://rpc.leanchain.io
NEXT_PUBLIC_EXPLORER_URL=https://explorer.leanchain.io
```

## Features

### Main Site (leanchain.io)
- Hero section with key metrics
- Feature showcase
- Service links
- Community links

### Forum (forum.leanchain.io)
- Category-based discussions
- Search functionality
- Recent posts listing
- User authentication (to be implemented)

### Documentation (docs.leanchain.io)
- Organized documentation sections
- Search functionality
- Quick links
- API references

### Validator Dashboard (validators.leanchain.io)
- Validator statistics
- Performance charts
- Validator list with details
- Staking distribution

### Block Explorer (explorer.leanchain.io)
- Recent blocks
- Recent transactions
- Search functionality
- Network statistics

### Staking Calculator (staking.leanchain.io)
- Interactive calculator
- Reward projections
- Validator recommendations
- APY calculations

### Network Status (status.leanchain.io)
- Service status monitoring
- Performance metrics
- Incident history
- Uptime statistics

## Customization

### Colors

Edit `tailwind.config.js` to customize the color scheme:

```js
colors: {
  primary: { /* ... */ },
  accent: { /* ... */ },
}
```

### Content

Update page components in `app/` directory to modify content.

## Contact

- **Developer**: Digital Trendz
- **Email**: dataswarmproject@gmail.com

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT / Apache-2.0 (same as main project)

