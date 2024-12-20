FROM ubuntu:24.04

# Workaround for macos: https://stackoverflow.com/questions/67732260/how-to-fix-hash-sum-mismatch-in-docker-on-mac
RUN echo "Acquire::http::Pipeline-Depth 0;" > /etc/apt/apt.conf.d/99custom && \
    echo "Acquire::http::No-Cache true;" >> /etc/apt/apt.conf.d/99custom && \
    echo "Acquire::BrokenProxy    true;" >> /etc/apt/apt.conf.d/99custom;

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Pacific/Auckland
<<<<<<< Updated upstream
RUN apt-get update && apt-get install -y curl tzdata sudo git && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install dotfiles.
RUN --mount=type=bind,source=.,target=/tmp/dotfiles,readonly \
    cp -r /tmp/dotfiles /tmp/dotfiles-copy && \
    cd /tmp/dotfiles-copy && \
    git submodule update --init --recursive --depth 2 && \
    ./configure.sh && \
    # Cleanup after installation
    rm -rf /tmp/dotfiles-copy && \
    # Cleanup the apt lists (again).
    apt-get clean && rm -rf /var/lib/apt/lists/*
=======

RUN apt-get update && \
    apt-get install -y curl tzdata sudo git software-properties-common zsh && \
    add-apt-repository universe && \
    apt-get update
    # Clean up
    # apt-get clean && \
    # rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Install dotfiles.
RUN --mount=type=bind,source=.,target=/tmp/dotfiles,readonly \
    /tmp/dotfiles/configure.sh "/tmp/dotfiles" && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /var/tmp/*
>>>>>>> Stashed changes

# Set the shell to zsh
RUN chsh -s /bin/zsh
ENTRYPOINT ["/bin/zsh"]
