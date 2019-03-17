#!/bin/sh

diesel migration $1 \
  --database-url postgres://developer:developer@localhost:5432/cqrs_example_$2 \
  --migration-dir migrations/$2 \
  --config-file diesel-$2.toml
echo "migration $1, db: $2 - Ok!"