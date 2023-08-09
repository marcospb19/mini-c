check: set_env
	@cargo check

run: set_env
	@cargo run

set_env:
	@export LLVM_SYS_150_PREFIX=$(realpath ../llvm/clang*/)

# Insta stuff
test:
	@cargo insta test

review:
	@cargo insta review

# Aliases for insta
t: test
r: review
