FROM mcr.microsoft.com/devcontainers/base:1.0.23-ubuntu-22.04

RUN curl -Lo /tmp/dotslash.tar.gz https://github.com/facebook/dotslash/releases/download/v0.4.1/dotslash-ubuntu-22.04.x86_64.v0.4.1.tar.gz \
    && echo "d3d587c707b9c7a8f5b7d1c26ec27ea16967cf2b5447728f0761a0292b6f789e  /tmp/dotslash.tar.gz" | sha256sum -c - \
    && tar -C /usr/local/bin -vxzf /tmp/dotslash.tar.gz \
    && rm /tmp/dotslash.tar.gz
