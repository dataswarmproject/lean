#!/bin/bash

# LEAN Chain Website Deployment Script

set -e

echo "🚀 Starting LEAN Chain website deployment..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

# Check Node.js version
NODE_VERSION=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$NODE_VERSION" -lt 18 ]; then
    echo "❌ Node.js version 18+ is required. Current version: $(node -v)"
    exit 1
fi

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Build the project
echo "🔨 Building the project..."
npm run build

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📋 Next steps:"
    echo "1. Start the production server: npm start"
    echo "2. Or use PM2: pm2 start npm --name 'leanchain-website' -- start"
    echo "3. Configure nginx using deploy/nginx.conf"
    echo "4. Set up SSL certificates with Let's Encrypt"
    echo ""
    echo "🌐 Subdomains configured:"
    echo "  - leanchain.io"
    echo "  - forum.leanchain.io"
    echo "  - docs.leanchain.io"
    echo "  - validators.leanchain.io"
    echo "  - explorer.leanchain.io"
    echo "  - staking.leanchain.io"
    echo "  - status.leanchain.io"
else
    echo "❌ Build failed!"
    exit 1
fi

