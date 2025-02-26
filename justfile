repo := 'ghcr.io/josiahbull/dotfiles'

# Install pre-commit
install:
    #!/usr/bin/env bash
    python3 -m venv venv
    source venv/bin/activate
    pip install --upgrade pip
    pip install pre-commit

# Run pre-commit on all files
pre-commit:
    #!/usr/bin/env bash
    source venv/bin/activate
    pre-commit run --all-files

# Build the Docker image, with the platform provided as an argument.
docker-build platform=(arch()):
    #!/usr/bin/env bash
    sha=$(git rev-parse --short HEAD)

    echo "Building image for platform: {{platform}}"
    echo "Docker version: $(docker --version)"
    echo "Docker buildx version: $(docker buildx version)"
    echo "Git SHA: $sha"

    docker buildx create --use
    docker buildx build \
        --platform "linux/{{platform}}" \
        -t "{{repo}}:{{platform}}-$sha" \
        -t "{{repo}}:{{platform}}-latest" \
        --load \
        .

# Push the Docker image to the GitHub Container Registry
docker-push platform=(arch()):
    #!/usr/bin/env bash
    sha=$(git rev-parse --short HEAD)

    echo "Pushing image for platform: {{platform}}"
    echo "Docker version: $(docker --version)"
    echo "Docker buildx version: $(docker buildx version)"
    echo "Git SHA: $sha"

    docker push "{{repo}}:{{platform}}-$sha"
    docker push "{{repo}}:{{platform}}-latest"

docker-manifest platforms=(arch()):
    #!/bin/bash

    set -o errexit -o nounset -o pipefail

    # Pull the images
    echo "Pulling images"
    for platform in {{platforms}}; do
        echo "Pulling {{repo}}:$platform-$(git rev-parse --short HEAD)"
        docker pull {{repo}}:$platform-$(git rev-parse --short HEAD);
        docker pull {{repo}}:$platform-latest;
    done

    # Create the manifest for sha
    echo "Creating manifest for {{repo}}:$(git rev-parse --short HEAD)"
    docker manifest create {{repo}}:$(git rev-parse --short HEAD) \
        $(for platform in {{platforms}}; do echo -n "--amend {{repo}}:$platform-$(git rev-parse --short HEAD) "; done)

    # Create the manifest for latest
    echo "Creating manifest for latest"
    docker manifest create {{repo}}:latest \
        $(for platform in {{platforms}}; do echo -n "--amend {{repo}}:$platform-latest "; done)

docker-manifest-push:
    #!/bin/bash

    set -o errexit -o nounset -o pipefail

    docker manifest push "{{repo}}:$(git rev-parse --short HEAD)"
    docker manifest push "{{repo}}:latest"

default:
    @just --list
