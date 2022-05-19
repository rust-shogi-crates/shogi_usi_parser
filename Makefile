all: sharedlib include c_tests

.PHONY: sharedlib include check-include c_tests

sharedlib:
	cargo +nightly build --release --no-default-features --features alloc

include: include/shogi_usi_parser.h

include/%.h: cbindgen.toml
	./generate_header.sh $*

check-include:
	./generate_header.sh shogi_usi_parser --verify

c_tests: sharedlib include
	$(MAKE) -C c_tests
