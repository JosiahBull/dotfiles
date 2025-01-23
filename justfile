# Install pre-commit
install:
    # Upgrade pip if necessary
    @pip install --upgrade pip
    @pip install pre-commit

# Run pre-commit on all files
pre-commit:
    @pre-commit run --all-files

# Build the Docker image, with the platform provided as an argument.
build platform="--":
    #!/usr/bin/env bash
    sha=$(git rev-parse --short HEAD)
    image_base_name=ghcr.io/josiahbull/dotfiles
    platform=$(echo {{platform}} | tr '[:upper:]' '[:lower:]')
    # if platform is not provided, default to the current platform.
    if [ -z "$platform" ]; then
        platform=$(shell uname -m)
        if [ "$platform" = "x86_64" ]; then
            platform=amd64
        fi
        platform=linux/$platform
    fi

    echo "Building image for platform: $platform"
    echo "Docker version: $(docker --version)"
    echo "Docker buildx version: $(docker buildx version)"
    echo "Git SHA: $sha"

    docker buildx create --use
    docker buildx build \
        --platform $platform \
        -t "$image_base_name:$sha-$platform" \
        -t "$image_base_name:latest-$platform" \
        --load \
        .

# Push the Docker image to the GitHub Container Registry
push platform="--":
    #!/usr/bin/env bash
    sha=$(git rev-parse --short HEAD)
    image_base_name=ghcr.io/josiahbull/dotfiles
    platform=$(echo {{platform}} | tr '[:upper:]' '[:lower')
    # if platform is not provided, default to the current platform.
    if [ -z "$platform" ]; then
        platform=$(shell uname -m)
        if [ "$platform" = "x86_64" ]; then
            platform=amd64
        fi
        platform=linux/$platform
    fi

    echo "Pushing image for platform: $platform"
    echo "Docker version: $(docker --version)"
    echo "Docker buildx version: $(docker buildx version)"
    echo "Git SHA: $sha"

    docker push "$image_base_name:$sha-$platform"
    docker push "$image_base_name:latest-$platform"

# Create a manifest for the Docker image
manifest:
    #!/usr/bin/env bash
    sha=$(git rev-parse --short HEAD)
    image_base_name=ghcr.io/josiahbull/dotfiles

    echo "Docker version: $(docker --version)"
    echo "Docker buildx version: $(docker buildx version)"
    echo "Git SHA: $sha"

    docker pull "$image_base_name:$sha-amd64"
    docker pull "$image_base_name:$sha-arm64"
    docker pull "$image_base_name:latest-amd64"
    docker pull "$image_base_name:latest-arm64"

    docker manifest create "$image_base_name:$sha" \
        --amend "$image_base_name:$sha-amd64" \
        --amend "$image_base_name:$sha-arm64"

    docker manifest create "$image_base_name:latest" \
        --amend "$image_base_name:latest-amd64" \
        --amend "$image_base_name:latest-arm64"

push-manifest:
    #!/usr/bin/env bash
    sha=$(git rev-parse --short HEAD)
    image_base_name=ghcr.io/josiahbull/dotfiles

    echo "Docker version: $(docker --version)"
    echo "Docker buildx version: $(docker buildx version)"
    echo "Git SHA: $sha"

    docker manifest push "$image_base_name:$sha"
    docker manifest push "$image_base_name:latest"
