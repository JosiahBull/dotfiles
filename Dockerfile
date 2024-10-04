FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Pacific/Auckland
# Intentionally not cleaning up here as I use the apt-lists when working.
RUN apt-get update && apt-get install -y curl tzdata sudo git

# Install dotfiles.
RUN --mount=type=bind,source=.,target=/dotfiles,readonly \
    cp -r /dotfiles /root/.dotfiles && \
    /root/.dotfiles/configure.sh "/root/.dotfiles" && \
    rm -rf /root/.dotfiles

# Set the shell to zsh
RUN chsh -s /bin/zsh
SHELL ["/bin/zsh", "-c"]
