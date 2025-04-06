ecmp:
	@cargo test --quiet ecmp -- --nocapture

matrix:
	@cargo test --quiet matrix -- --nocapture

repair:
	@cargo test --quiet repair -- --nocapture
