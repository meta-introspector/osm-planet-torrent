# Quality Management System - Makefile
# Standards: Six Sigma, ITIL, GMP, ISO 9001

.PHONY: all test coverage lint audit validate clean

# Six Sigma: Define phase
all: validate test coverage

# Six Sigma: Measure phase
test:
	@echo "Running unit tests (Target: 100% pass)..."
	cargo test --all-features

# Six Sigma: Analyze phase
coverage:
	@echo "Measuring code coverage (Target: 70%)..."
	cargo tarpaulin --out Html --output-dir target/coverage

# GMP: Quality Control
lint:
	@echo "Running quality checks..."
	cargo clippy -- -D warnings
	cargo fmt -- --check

# ISO 9001: Evidence-based
audit:
	@echo "Generating audit trail..."
	@echo "Git commits:" > audit_trail.txt
	git log --oneline -10 >> audit_trail.txt
	@echo "\nTest results:" >> audit_trail.txt
	cargo test 2>&1 | tee -a audit_trail.txt

# ITIL: Service Validation
validate: lint test
	@echo "✓ Validation complete"

# Six Sigma: Control phase
monitor:
	@echo "Performance metrics:"
	@cargo bench

clean:
	cargo clean
	rm -f audit_trail.txt
