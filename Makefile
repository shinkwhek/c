all: src/*
	cargo build

test: src/*
	cargo test
	./test.bash

clean:
	rm -f tmp*
