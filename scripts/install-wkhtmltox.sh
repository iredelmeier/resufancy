#!/bin/bash

set -eu

mkdir -p /tmp
cd /tmp || exit 1
curl -L https://github.com/wkhtmltopdf/wkhtmltopdf/releases/download/0.12.3/wkhtmltox-0.12.3_linux-generic-amd64.tar.xz | tar xvJ
cp wkhtmltox/lib/*.so* /usr/lib
