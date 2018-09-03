all: src/*
	cargo build

test: src/* test.bash
	cargo test
	./test.bash

clean:
	rm -f tmp*
