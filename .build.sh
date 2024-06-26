#!/bin/bash

# Install sudo if not already installed
if ! command -v sudo &>/dev/null; then
    apt-get update -qq
    apt-get install sudo -y
fi

# Check if the temporary user already exists
TEMP_USER="tempuser"
if ! id -u $TEMP_USER > /dev/null 2>&1; then
  useradd -m -s /bin/bash $TEMP_USER
  echo "$TEMP_USER ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
fi

# Create a temporary directory
TEMP_DIR=$(mktemp -d -p /tmp)
chown $TEMP_USER:$TEMP_USER $TEMP_DIR

# Run the rest of the script as the temporary user
sudo -u $TEMP_USER bash << EOF
set -e

arch=\$(dpkg --print-architecture)

cdir=\$(pwd)

build="${TEMP_DIR}/build"
location="\${build}/lib/python3/dist-packages"

mkdir -p \${build}
mkdir -p \${location}

package="\${build}/python3-pymainprocess-\${arch}.deb"

virtualenv .venv

source .venv/bin/activate

pip install pymainprocess

name=\$(pip show pymainprocess | grep "Name: " | awk -F "Name: " '{print \$2}')
version=\$(pip show pymainprocess | grep "Version: " | awk -F "Version: " '{print \$2}')
description=\$(pip show pymainprocess | grep "Summary: " | awk -F "Summary: " '{print \$2}')
maintainer=\$(pip show pymainprocess | grep "Author: " | awk -F "Author: " '{print \$2}')

cd "\${build}"

pip install pymainprocess --target "\${location}"

size=\$(du -sk "\${build}/lib" | awk '{print \$1}')

mkdir -p DEBIAN
cat <<CONTROL > DEBIAN/control
Package: \${name}
Version: \${version}
Section: utils
Priority: optional
Architecture: \${arch}
Installed-Size: \${size}
Depends: python3
Maintainer: \${maintainer}
Description: \${description}

CONTROL

dpkg-deb --build \${build} \${package}

cd  "\${cdir}"
EOF

# Move the created package to the expected directory
mv ${TEMP_DIR}/build/python3-pymainprocess-*.deb build/
chown $TEMP_USER:$TEMP_USER build/python3-pymainprocess-*.deb
rm -rf ${TEMP_DIR}