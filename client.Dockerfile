# ====================================================================
# =                                                                  =
# =                        NodeForum Frontend                        =
# =                                                                  =
# ====================================================================

# Create the NodeForum builder
FROM node:latest

# Add the packages
ADD ./packages/frontend /app
WORKDIR /app

# Install PNPM
RUN corepack enable

# Install dependencies
RUN pnpm install
RUN pnpm build

# Run the app
CMD [ "pnpm", "dev" ]
