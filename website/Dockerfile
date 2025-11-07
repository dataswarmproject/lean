# Build stage
FROM node:18-alpine AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY website/package*.json ./website/

# Install dependencies
RUN npm ci

# Copy source code
COPY website ./website
WORKDIR /app/website

# Build the application
RUN npm run build

# Production stage
FROM node:18-alpine AS runner

WORKDIR /app

ENV NODE_ENV=production

# Copy built application
COPY --from=builder /app/website/.next ./.next
COPY --from=builder /app/website/public ./public
COPY --from=builder /app/website/package*.json ./
COPY --from=builder /app/website/node_modules ./node_modules
COPY --from=builder /app/website/next.config.js ./

EXPOSE 3000

CMD ["npm", "start"]

