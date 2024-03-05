# Use the official Arch Linux image
FROM archlinux:latest

# Install necessary packages and update system
RUN pacman -Syu --noconfirm \
  && pacman -S --noconfirm \
  git \
  rust \
  base-devel

# Initialize pacman keyring and populate archlinux keys
RUN pacman-key --init \
  && pacman-key --populate archlinux

# Update system packages
RUN pacman -Syyuu --noconfirm

# Clone the repository
RUN git clone https://github.com/TheBearodactyl/discord-autodeleter /discord-autodeleter

# Add .env file to the Docker image
ADD .env /discord-autodeleter/

# Set working directory
WORKDIR /discord-autodeleter

# Check if the release directory exists
RUN if [ -d "/discord-deleter/target/release" ]; then \
  /discord-autodeleter/target/release/fuckyouspammer; \
  else \
  cargo build --release -j16; \
  fi

# Check if RUN environment variable is set to true, then execute the executable
CMD if [ "$RUN" = "true" ]; then \
  /discord-autodeleter/target/release/fuckyouspammer; \
  fi
