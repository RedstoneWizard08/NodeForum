# ====================================================================
# =                                                                  =
# =                        NodeForum Frontend                        =
# =                                                                  =
# ====================================================================

# Create the NodeForum builder
FROM node:latest

# Add the packages
ADD . /app
WORKDIR /app

# Install dependencies
RUN pnpm install
RUN pnpm run build
