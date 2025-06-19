FROM node:20-alpine

WORKDIR /build

# Pre-install common dependencies to speed up builds
RUN npm install -g rollup @rollup/plugin-typescript typescript

# Create a non-root user
RUN addgroup -g 1001 -S nodejs && adduser -S nodejs -u 1001

# Set npm cache directory
RUN npm config set cache /tmp/.npm

USER nodejs

CMD ["echo", "Node.js build environment ready"]