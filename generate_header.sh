# Must be run in the crate's root directory
CRATE_NAME=$1
shift

cbindgen --config cbindgen.toml --parse-dependencies --crate ${CRATE_NAME} --output include/${CRATE_NAME}.h $@
