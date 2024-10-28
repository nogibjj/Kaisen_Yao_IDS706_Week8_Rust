setup:
	cd python_proj && \
	make extract && \
	make transform_load

rust_version:
	cd rust_proj && \
	make rust-version

rust_format:
	cd rust_proj && \
	cargo fmt --quiet

rust_install:
	cd rust_proj && \
	make install

rust_lint:
	cd rust_proj && \
	cargo clippy --quiet

rust_test:
	cd rust_proj && \
	cargo test --quiet

rust_run:
	cd rust_proj && \
	cargo run

rust_release:
	cd rust_proj && \
	cargo build --release

rust_all:
	cd rust_proj && \
	make all

python_install:
	cd python_proj && \
	make install

python_test:
	cd python_proj && \
	make test

python_format:	
	cd python_proj && \
	make format

python_lint:
	cd python_proj && \
	make lint

python_refactor:
	cd python_proj && \
	make refactor

python_container-lint:
	cd python_proj && \
	make container-lint

python_run:
	cd python_proj && \
	make query

python_all:
	cd python_proj && \
	make all

generate_and_push:
	# Add, commit, and push the generated files to GitHub
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add metric log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi

run_both:
	cd rust_proj && \
	make run && \
	cd ../python_proj && \
	make query