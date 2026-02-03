FROM ubuntu:24.04

# Workaround for devcontainer: https://askubuntu.com/questions/1513927/ubuntu-24-04-docker-images-now-includes-user-ubuntu-with-uid-gid-1000
RUN touch /var/mail/ubuntu && chown ubuntu /var/mail/ubuntu && userdel -r ubuntu

# Workaround for macos: https://stackoverflow.com/questions/67732260/how-to-fix-hash-sum-mismatch-in-docker-on-mac
RUN echo "Acquire::http::Pipeline-Depth 0;" > /etc/apt/apt.conf.d/99custom && \
    echo "Acquire::http::No-Cache true;" >> /etc/apt/apt.conf.d/99custom && \
    echo "Acquire::BrokenProxy    true;" >> /etc/apt/apt.conf.d/99custom;

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Pacific/Auckland
ENV DOTFILES_DIR=/root/.dotfiles
RUN apt-get update && apt-get install -y curl tzdata sudo git software-properties-common zsh \
    pkg-config libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install dotfiles to ~/.dotfiles (persistent for linking)
RUN --mount=type=bind,source=.,target=/tmp/dotfiles,readonly \
    cp -r /tmp/dotfiles "$DOTFILES_DIR" && \
    cd "$DOTFILES_DIR" && \
    git submodule update --init --recursive --depth 2 && \
    ./configure.sh && \
    # Cleanup the apt lists (again).
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Note: pre-commit is already installed via pipx in configure.sh

# Set the shell to zsh
RUN chsh -s /bin/zsh
ENTRYPOINT ["/bin/zsh"]
