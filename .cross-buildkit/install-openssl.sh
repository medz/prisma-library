#!/usr/bin/env bash

# Install LibreSSL
apt-get update && apt-get install libssl-dev -y

# Link opensslconf.h
if [ ! -f /usr/include/openssl/opensslconf.h ]; then
    ln -s /usr/include/x86_64-linux-gnu/openssl/opensslconf.h /usr/include/openssl/opensslconf.h
fi

# Link configuration.h
if [ ! -f /usr/include/openssl/configuration.h ]; then
    ln -s /usr/include/x86_64-linux-gnu/openssl/configuration.h /usr/include/openssl/configuration.h
fi
