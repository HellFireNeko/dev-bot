# Use the Arch Linux base image
FROM archlinux/archlinux

# Update package repositories and install necessary dependencies
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm base-devel rustup gcc nodejs

# Set working directory for the application
WORKDIR /app

# Copy the application source code into the container
COPY . .

# Install/Update the rust toolchain to be the latest stable release
RUN rustup default stable

# Build the Rust application (use --release for optimized build)
RUN cargo build --release

# Expose the port used by the application (if applicable)
EXPOSE 80

# Set the entry point to run the compiled binary when the container starts
CMD ["./target/release/dev-bot"]