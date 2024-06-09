#!/bin/bash -e

PROFILE=${1:-debug}

if test "${PROFILE}" == "release" ; then
  ARGUMENTS="build --release"
elif test "${PROFILE}" == "debug" ; then
  ARGUMENTS="build"
elif test "${PROFILE}" == "doc" ; then
  ARGUMENTS="doc --no-deps"
else
  echo "Invalid argument \"${PROFILE}\" (debug, release, doc)"
  exit 1
fi

# Figure out where NodeJS' "include" directory is
NODE_DIR="$(node -p 'path.resolve(process.argv[0], "..", "..")')"
NODE_LIB_DIR="${NODE_DIR}/lib" # not used, but stil...
NODE_INCLUDE_DIR="${NODE_DIR}/include/node"

# We want to link against the OpenSSL library *bundled* within NodeJS. This
# shrinks our total binary size from ~7.8MB, to ~2.8MB (on MacOS, debug). Plus
# we don't have to have TWO copies of the same library in memory...
#
#   RUSTFLAGS="-C link-args=-Wl,-undefined,dynamic_lookup"
#     Ignore any undefined symbol, anything missing will pop up when loading
#     our library via "require". Specifically, we don't link against OpenSSL!
#
#   OPENSSL_LIB_DIR="${NODE_LIB_DIR}"
#     Find NodeJS libraries here... Not used, but still we need to specify a
#     valid directory so that "openssl-sys" and the linker won't complain!
#
#   OPENSSL_INCLUDE_DIR="${NODE_INCLUDE_DIR}"
#     This is where we'll find the OpenSSL headers provided by NodeJS itself!
#
#   OPENSSL_STATIC="0"
#     This basically tells "openssl-sys" not to check for anything in OpenSSL!
#
#   OPENSSL_LIBS=""
#     This tells "openssl-sys" that we don't have any -llib library to link!
#
RUSTFLAGS="-C link-args=-Wl,-undefined,dynamic_lookup" \
OPENSSL_LIB_DIR="${NODE_LIB_DIR}" \
OPENSSL_INCLUDE_DIR="${NODE_INCLUDE_DIR}" \
OPENSSL_STATIC="0" \
OPENSSL_LIBS="" \
  cargo ${ARGUMENTS}

# Bail out if we're building docs
echo "$PROFILE"
if test "${PROFILE}" = "doc" ; then
  exit 0
fi

# Copy our library file here, for testing...
cp "./target/${PROFILE}/libpq_rs_node.dylib" ./libpq_rs.node

# Run!
RUST_BACKTRACE=1 node ./runme.js
