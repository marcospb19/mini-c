# Aliases
t: test
r: review

test:
	@cargo insta test

review:
	@cargo insta review

setup:
	@echo 'Installing git hook'
	cp scripts/git_hooks/* .git/hooks/
