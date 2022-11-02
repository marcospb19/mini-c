# This should run on pre-commit
export INSTA_SNAPSHOT_REFERENCES_FILE="$(mktemp)"
cargo test
rg --files -lg '*.snap' "$(pwd)" | grep -vFf "$INSTA_SNAPSHOT_REFERENCES_FILE" | xargs rm
rm -f $INSTA_SNAPSHOT_REFERENCES_FILE
