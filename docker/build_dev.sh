#!/bin/bash

docker build \
    --build-arg UID=$(id -u) \
    --build-arg GID=$(id -g) \
    --build-arg USERNAME=$(whoami) \
    -f docker/Dockerfile.dev \
    -t tr8tr-rs-dev:latest .
