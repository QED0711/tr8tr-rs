#!/bin/bash

USERNAME=$(whoami)

docker run --rm -ti -d \
    --name tr8tr_rs_dev \
    -v $PWD/:/home/$USERNAME/app \
    tr8tr-rs-dev:latest tail -f /dev/null
