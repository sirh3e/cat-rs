CARGO	:= cargo

.PHONY: all
all: fmt test run

.PHONY: run
run:
	$(CARGO) $@

.PHONY: build
build:
	$(CARGO) $@

.PHONY: test
test:
	$(CARGO) $@

.PHONY: fmt
fmt:
	$(CARGO) $@

.PHONY: clean
clean:
	$(CARGO) $@

.PHONY: release
release:
	$(CARGO) build --$@

.PHONY: diff
diff:
	./diff.sh
