.PHONY: test-all
test-all:
	cargo test --workspace

.PHONY: test-parser
test-parser:
	cargo test -p parser

.PHONY: test-automaton
test-automaton:
	cargo test -p automaton

.PHONY: test-virtual-machine
test-virtual-machine:
	cargo test -p virtual_machine
