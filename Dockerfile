# Stage 1: Build the application
FROM rustlang/rust:nightly as builder

# Install dependencies required for Diesel CLI
RUN apt-get update && apt-get install -y \
    libpq-dev \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy all the files to the working directory
COPY . .

# Build dependencies
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/superteam-earn-community .

# Specify the command to run the application
CMD ["./superteam-earn-community"]
