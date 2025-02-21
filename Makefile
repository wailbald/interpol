# Macros
CC=mpicc
CFLAGS=-Wall -Wextra -g3
OFLAGS=-march=native -mtune=native -Os

INCLUDE=include
RS_SRC=interpol-rs/src
RS_LIB=interpol-rs/target/release
SRC=src

PWD=$(shell pwd)

build: libinterpol.so
	@export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(PWD)/interpol-rs/target/release/:$(PWD)

install: build
	@cp libinterpol.so interpol-rs/target/release/libinterpol_rs.so /usr/lib/

uninstall:
	@rm /usr/lib/libinterpol.so /usr/lib/libinterpol_rs.so 

libinterpol.so: $(RS_LIB)/libinterpol_rs.so $(INCLUDE)/tsc.h $(SRC)/interpol.c $(INCLUDE)/interpol.h
	$(CC) $(CFLAGS) $(OFLAGS) -I$(INCLUDE) -L$(RS_LIB) -fPIC -shared $(SRC)/interpol.c -o $@ -linterpol_rs

$(RS_LIB)/libinterpol_rs.so: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo build --release

test: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo test
	
doc: $(RS_SRC)/*.rs
	@cd interpol-rs/ && cargo doc --document-private-items --open

reset:
	@rm -Rf $(TARGET) libinterpol.so

clean:
	@cd interpol-rs/ && cargo clean
	@rm -Rf $(TARGET) libinterpol.so

.PHONY: build install uninstall test doc reset clean
