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

target/release/examples/performance_dnarna:
	cargo build --release --example performance_dnarna

profile: target/release/examples/performance_selfcheck
	cp callgrind.annotate callgrind.annotate.`date '+%Y%m%d%H%M%S'`
	cargo build --release --example performance_selfcheck
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< >/dev/null
	callgrind_annotate --auto=yes --inclusive=yes --tree=caller callgrind.profile > callgrind.annotate
	less callgrind.annotate

profile_heavy: target/release/examples/performance_dnarna
	cargo build --release --example performance_dnarna
	cp callgrind.annotate callgrind.annotate.`date '+%Y%m%d%H%M%S'`
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $<
	callgrind_annotate --auto=yes --inclusive=yes --tree=caller callgrind.profile > callgrind.annotate
	less callgrind.annotate
