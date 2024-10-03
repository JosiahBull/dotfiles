FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Pacific/Auckland
# Intentionally not cleaning up here as I use the apt-lists when working.
RUN apt-get update && apt-get install -y curl tzdata

# Install dotfiles.
RUN curl https://raw.githubusercontent.com/JosiahBull/dotfiles/main/install.sh | bash

# Set the shell to zsh
RUN chsh -s /bin/zsh
SHELL ["/bin/zsh", "-c"]
