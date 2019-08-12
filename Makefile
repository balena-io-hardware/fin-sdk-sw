HEADERS = \
	include/fin/fin.h \
	include/fin/client.h

RUST_SOURCES = \
	fin-cli/src/command.rs \
	fin-cli/src/main.rs \
	fin-lib/src/ethtool.rs \
	fin-lib/src/i2c.rs \
	fin-lib/src/lib.rs \
	fin-sdk/src/lib.rs

all: Fin-0.2.gir Fin-0.2.typelib

target/release/libfin.so: $(RUST_SOURCES)
	cargo build --release

Fin-0.2.gir: target/release/libfin.so $(HEADERS)
	g-ir-scanner -v --warn-all --warn-error \
		--namespace Fin --nsversion=0.2 \
		-Iinclude --c-include "fin/fin.h" \
		--library=fin --library-path=$(PWD)/target/release \
		--include=GObject-2.0 -pkg gobject-2.0 \
		--output $@ \
		$(HEADERS)

Fin-0.2.typelib: Fin-0.2.gir
	g-ir-compiler --includedir=include $< -o $@

clean:
	rm -f Fin-0.2.typelib
	rm -f Fin-0.2.gir
	cargo clean

py: Fin-0.2.typelib
	GI_TYPELIB_PATH=$(PWD) LD_LIBRARY_PATH=$(PWD)/target/release python3 examples/py/properties/properties.py

js: Fin-0.2.typelib
	GI_TYPELIB_PATH=$(PWD) LD_LIBRARY_PATH=$(PWD)/target/release node examples/js/revision/revision.js

