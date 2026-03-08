.PHONY: check watch clean

check:
	cargo check
	cargo clippy -- -D warnings
	typos

watch:
	@bash -c 'source .env ; export $$(grep -v '^#' .env | sed 's/=.*//') ; bacon --headless'
