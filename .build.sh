#!/bin/bash

arch="$1"

cdir=$(pwd)

build="${cdir}/build"
location="${build}/lib/python3/dist-packages"

mkdir -p ${build}
mkdir -p ${location}

package="${build}/python3-pymainprocess.deb"

virtualenv .venv

source .venv/bin/activate

pip install pymainprocess

name=$(pip show pymainprocess | grep "Name: " | awk -F "Name: " '{print $2}')
version=$(pip show pymainprocess | grep "Version: " | awk -F "Version: " '{print $2}')
description=$(pip show pymainprocess | grep "Summary: " | awk -F "Summary: " '{print $2}')
maintainer=$(pip show pymainprocess | grep "Author: " | awk -F "Author: " '{print $2}')

cd "${build}"

pip install pymainprocess --target "${location}"

size=$(du -sk "${build}/lib" | awk '{print $1}')

cat <<EOF > control
Package: ${name}
Version: ${version}
Section: utils
Priority: optional
Architecture: ${arch}
Installed-Size: ${size}
Depends: python3
Maintainer: ${maintainer}
Description: ${description}

EOF

tar -cJf control.tar.xz ./control
tar -cJf data.tar.xz ./lib

rm -rf ./control ./lib

echo 2.0 > debian-binary

ar rcs "${package}" debian-binary control.tar.xz data.tar.xz

rm -rf debian-binary control.tar.xz data.tar.xz 

cd  "${cdir}"