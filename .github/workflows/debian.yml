name: Debian Package

on:
  workflow_dispatch:

jobs:
  linux:
    runs-on: ${{ matrix.platform.runner }}
    strategy:
      matrix:
        platform:
          - runner: ubuntu-latest
            target: x86_64
          - runner: ubuntu-latest
            target: x86
          - runner: ubuntu-latest
            target: aarch64
          - runner: ubuntu-latest
            target: armv7
          - runner: ubuntu-latest
            target: s390x
          - runner: ubuntu-latest
            target: ppc64le
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
      - name: Create Debian Package
        run: |
          sudo apt-get update -qq
          sudo apt-get install -y tar xz-utils binutils python3-virtualenv
          mkdir build
          arch=$(dpkg --print-architecture)
          ./.build.sh ${arch}
          mv build/python3-pymainprocess.deb build/python3-pymainprocess-${{ matrix.platform.target }}.deb
      - name: Upload Debian Package
        uses: actions/upload-artifact@v4
        with:
          name: python3-pymainprocess-${{ matrix.platform.target }}.deb
          path: build/python3-pymainprocess-${{ matrix.platform.target }}.deb