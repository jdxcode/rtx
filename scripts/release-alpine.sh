#!/usr/bin/env bash
set -euxo pipefail

RTX_VERSION=$(./scripts/get-version.sh)

su - packager
cd

#SHA512=$(curl -L "https://github.com/jdxcode/rtx/archive/$RTX_VERSION.tar.gz" | sha512sum | awk '{print $1}')

sudo sh -c "echo \"$ALPINE_PUB_KEY\" > /etc/apk/keys/-640e56d3.rsa.pub"
mkdir -p /home/packager/.abuild
echo "$ALPINE_PRIV_KEY" >/home/packager/.abuild/-640e56d3.rsa

if [ ! -d aports ]; then
	git clone https://gitlab.alpinelinux.org/alpine/aports.git aports
fi
cd aports
git config --local core.hooksPath .githooks
cd testing/rtx
git pull

sed -i "s/pkgver=.*/pkgver=${RTX_VERSION#v*}" APKBUILD
abuild checksum
abuild -r
apkbuild-lint APKBUILD

git add APKBUILD
git commit -m "testing/rtx: $RTX_VERSION"
git show

git remote add jdxcode "https://jdxcode:$GITLAB_TOKEN@gitlab.alpinelinux.org/jdxcode/aports.git"
git push -f jdxcode "HEAD:testing/rtx/$RTX_VERSION"
