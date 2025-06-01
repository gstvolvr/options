#!/bin/bash

# install `mkcert` locally
brew install mkcert


# install local CA (Certificate Authority) in system's trust store
mkcert --install 
# generate certificate for localhost
mkcert 127.0.0.1
