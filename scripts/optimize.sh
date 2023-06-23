ARM_VERSION="0.13.0"
INTEL_VERSION="0.13.0"

function optimize_arm() {
  docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:$ARM_VERSION
}

function optimize_intel() {
  docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:$INTEL_VERSION
}

if [[ $(uname -m) =~ "arm64" ]]; then \
  optimize_arm; \
else \
  optimize_intel; \
fi
