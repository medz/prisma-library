#!/usr/bin/env bash

# Install LibreSSL
apt-get update && apt-get install libssl-dev -y

# Link configuration.h and opensslconf.h
ln -s /usr/include/x86_64-linux-gnu/openssl/opensslconf.h /usr/include/openssl/opensslconf.h
ln -s /usr/include/x86_64-linux-gnu/openssl/opensslconf.h /usr/include/openssl/opensslconf.h
