# Dockerfile for Smithery.ai deployment
# https://smithery.ai/docs/build/project-config/smithery.yaml

FROM node:20-alpine

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY bun.lock* ./

# Install dependencies
RUN npm install --production

# Copy source and build files
COPY src/ ./src/
COPY dist/ ./dist/
COPY tsconfig.json ./

# Set default environment variables
ENV NODE_ENV=production
ENV LOG_LEVEL=info

# The entry point - Smithery will override with commandFunction
CMD ["node", "./dist/index.js"]
