# Use the Arch Linux base image
FROM archlinux:latest

# Update and install necessary packages
RUN pacman -Syu --noconfirm && \
  pacman -S --noconfirm pacman-contrib git rust base-devel

# Initialize and populate pacman keyring, update system
RUN pacman-key --init && pacman-key --populate archlinux && pacman -Syyuu --noconfirm

# Clone the repository
RUN git clone https://github.com/TheBearodactyl/discord-autodeleter /discord-autodeleter

# Copy .env file to the specified directory
COPY /home/emi/.local/share/junk/projects/fuckyouspammer/.env /discord-autodeleter

# Set working directory
WORKDIR /discord-autodeleter

# Check if the release directory exists
RUN if [ -d "/discord-autodeleter/target/release" ]; then \
  /discord-autodeleter/target/release/fuckyouspammer; \
  else \
  cargo build --release -j16 && \
  /discord-autodeleter/target/release/fuckyouspammer; \
  fi
