BIN=./target/release/

all:
	cargo build --release
	cp $(BIN)/latte latc_llvm

clean:
	- cargo clean
	rm -rf latc_llvm
	- $(CARGO) clean
	- rm -rf cargo
