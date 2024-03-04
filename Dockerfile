FROM archlinux:latest

CMD [ "rm", "-fr", "/discord-autodeleter" ]

RUN pacman-key --init && pacman-key --populate archlinux && pacman -Syyuu --noconfirm
RUN pacman -Syyuu git rust base-devel --noconfirm
RUN git clone https://github.com/TheBearodactyl/discord-autodeleter
WORKDIR /discord-autodeleter

RUN if [[ -d /discord-autodeleter/target/release ]] ; then exec ./target/release/fuckyouspammer ; else cargo build --release -j16 && exec /discord-autodeleter/target/release/discord-autodeleter ; fi
