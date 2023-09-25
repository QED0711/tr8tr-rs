#!/bin/bash

USERNAME=$(whoami)

docker run --rm -ti \
    -v $PWD/:/home/$USERNAME/app \
    tr8tr-rs-dev:latest bash