FROM ubuntu:22.04

# Workaround for macos: https://stackoverflow.com/questions/67732260/how-to-fix-hash-sum-mismatch-in-docker-on-mac
RUN echo "Acquire::http::Pipeline-Depth 0;" > /etc/apt/apt.conf.d/99custom && \
    echo "Acquire::http::No-Cache true;" >> /etc/apt/apt.conf.d/99custom && \
    echo "Acquire::BrokenProxy    true;" >> /etc/apt/apt.conf.d/99custom;

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Pacific/Auckland
# Intentionally not cleaning up here as I use the apt-lists when working.
RUN apt-get update && apt-get install -y curl tzdata sudo git

# Install dotfiles.
RUN --mount=type=bind,source=.,target=/tmp/dotfiles,readonly \
    /tmp/dotfiles/configure.sh "/tmp/dotfiles"

# Set the shell to zsh
RUN chsh -s /bin/zsh
SHELL ["/bin/zsh", "-c"]
