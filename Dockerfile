# Setup Stage - set up the ZSH environment for optimal developer experience
FROM node:14-alpine AS setup
# Let scripts know we're running in Docker (useful for containerised development)
ENV RUNNING_IN_DOCKER true
# Use the unprivileged `node` user (pre-created by the Node image) for safety (and because it has permission to install modules)
RUN mkdir -p /app \
    && chown -R node:node /app
# Set up ZSH and our preferred terminal environment for containers
RUN apk --no-cache add zsh curl git
RUN mkdir -p /home/node/.antigen
RUN curl -L git.io/antigen > /home/node/.antigen/antigen.zsh
# Use my starter Docker ZSH config file for this, or your own ZSH configuration file (https://gist.github.com/arctic-hen7/bbfcc3021f7592d2013ee70470fee60b)
COPY .dockershell.sh /home/node/.zshrc
RUN chown -R node:node /home/node/.antigen /home/node/.zshrc
# Set up ZSH as the unprivileged user (we just need to start it, it'll initialise our setup itself)
USER node
RUN /bin/zsh /home/node/.zshrc
# Switch back to root for whatever else we're doing
USER root

# Rust Setup Stage - install and set up Rust for development (used for backend)
FROM setup AS rust-setup
# Install the necessary system dependencies
RUN apk add --no-cache build-base clang llvm gcc
# Download and run the Rust installer, using the default options (needs to be done as the unprivileged user)
USER node
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Install Cargo plugins
# We have to use the absolute path to the Cargo binary because it isn't aliased in the Docker build process
RUN /home/node/.cargo/bin/cargo install cargo-watch
# Switch back to root for the remaining stages
USER root

# Dependencies Stage - install all system-level dependencies that won't change (before Rust caching because that gets constantly re-executed)
FROM rust-setup AS dependencies
# Install system dependencies
USER root
RUN apk add --no-cache openssl-dev
# Install global dependencies with NPM
# See https://answers.netlify.com/t/netlify-cli-fails-to-install/34508/3 for why we use `--unsafe-perm`
RUN npm install -g --unsafe-perm netlify-cli

# Rust Cacher Stage - caches all dependencies in the Rust code with `cargo vendor` to speed up builds massively
# When your dependencies change, this will be re-executed, otherwise you get super-speed caching performance!
FROM dependencies AS rust-cacher
USER node
RUN mkdir -p /app \
    && chown -R node:node /app
# Copy the Cargo configuration files into the correct place in the container
# Note that we need to be able to write to Cargo.lock
WORKDIR /app
COPY --chown=node:node ./Cargo.lock Cargo.lock
COPY ./Cargo.toml Cargo.toml
# We also copy over all the manifests of all the integrations (workspace structure)
COPY ./integrations/serverful/actix-web/Cargo.toml integrations/serverful/actix-web/Cargo.toml
COPY ./integrations/serverless/aws-lambda/Cargo.toml integrations/serverless/aws-lambda/Cargo.toml
# Vendor all dependencies (stores them all locally, meaning they can be cached)
RUN mkdir -p /app/.cargo
RUN chown -Rh node:node /app/.cargo
RUN /home/node/.cargo/bin/cargo vendor > .cargo/config
# Switch back to root for the remaining stages
USER root

# Base Stage - install system-level dependencies, disable telemetry, and copy files
FROM rust-cacher AS base
WORKDIR /app
# Disable telemetry of various tools for privacy
RUN yarn config set --home enableTelemetry 0
# Copy the Netlify config file into the correct location
# See `CONTRIBUTING.md` for how to set this up for the first time
COPY --chown=node:node ./netlify-config.json /home/node/.config/netlify/config.json
# Copy our source code into the container
COPY . .

# Playground stage - simple ZSH entrypoint for us to shell into the container as the non-root user for developing the main library
FROM base AS playground
USER node
ENTRYPOINT [ "/bin/zsh" ]
