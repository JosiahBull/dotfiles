repo := 'ghcr.io/josiahbull/dotfiles'

# Install pre-commit
install:
    # Upgrade pip if necessary
    @pip install --upgrade pip
    @pip install pre-commit

# Run pre-commit on all files
pre-commit:
    @pre-commit run --all-files

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
        -t "{{repo}}:$sha-{{platform}}" \
        -t "{{repo}}:latest-{{platform}}" \
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

    docker push "{{repo}}:$sha-{{platform}}"
    docker push "{{repo}}:latest-{{platform}}"

docker-manifest triples=(arch()):
    #!/bin/bash

    set -o errexit -o nounset -o pipefail

    # Pull the images
    echo "Pulling images"
    for triple in {{triples}}; do
        echo "Pulling {{repo}}:$triple-$(git rev-parse --short HEAD)"
        docker pull {{repo}}:$triple-$(git rev-parse --short HEAD);
        docker pull {{repo}}:$triple-latest;
    done

    # Create the manifest for sha
    echo "Creating manifest for {{repo}}:$(git rev-parse --short HEAD)"
    docker manifest create {{repo}}:$(git rev-parse --short HEAD) \
        $(for triple in {{triples}}; do echo -n "--amend {{repo}}:$triple-$(git rev-parse --short HEAD) "; done)

    # Create the manifest for latest
    echo "Creating manifest for latest"
    docker manifest create {{repo}}:latest \
        $(for triple in {{triples}}; do echo -n "--amend {{repo}}:$triple-latest "; done)

docker-manifest-push:
    #!/bin/bash

    set -o errexit -o nounset -o pipefail

    docker manifest push "{{repo}}:$(git rev-parse --short HEAD)"
    docker manifest push "{{repo}}:latest"

default:
    @just --list
