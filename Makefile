help:
	$(info -Targets---------------------------------------)
	$(info -Development Targets --------------------------)
	$(info lint                  | run lints with clippy)
	$(info fmt                   | format src)
	$(info profile               | run valgrind callgrind)

fix:
	cargo fix

fmt:
	cargo fmt

lint:
	cargo clippy

profile: target/release/examples/performance_selfcheck
	cargo build --release --example performance_selfcheck
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< >/dev/null
	callgrind_annotate --auto=yes --inclusive=yes --tree=caller callgrind.profile > callgrind.annotate
	less callgrind.annotate
