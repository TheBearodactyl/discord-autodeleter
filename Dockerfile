FROM archlinux:latest

RUN pacman-key --init && pacman-key --populate archlinux && pacman -Syyuu --noconfirm
RUN pacman -Syyuu git rust base-devel --noconfirm
RUN git clone https://github.com/TheBearodactyl/discord-autodeleter
WORKDIR /discord-autodeleter
RUN cargo build --release -j16
RUN exec ./target/release/fuckyouspammer
