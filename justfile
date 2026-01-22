check:
  cargo fmt -- --check
  cargo clippy --all-targets -- -D warnings
  cargo check --all-features

# Run a test suite: unit
test:
  cargo test
  cargo test --doc
  cargo test --examples

# Delete unused files or branches: data, lockfile, branches
delete item="branches":
  just _delete-{{item}}

_delete-lockfile:
  rm -f Cargo.lock

_delete-branches:
  git branch --merged | grep -v \* | xargs git branch -d
