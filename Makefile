compiler=cargo

source_file=./src/main.rs

.PHONY: run

run: test
	$(compiler) $@

build: test
	$(compiler) $@

test: fmt
	$(compiler) $@

fmt: clean
	$(compiler) $@

clean:
	$(compiler) $@

release:
	$(compiler) build --release

diff:
	./diff.sh
