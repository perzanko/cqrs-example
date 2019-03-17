#!/bin/sh

. .env

cmd=$1
dbtype=$2

if [ $dbtype = "read" ]
then
  connection=$DATABASE_URL_READ
else
  connection=$DATABASE_URL_WRITE
fi

diesel migration $cmd \
  --database-url $connection \
  --migration-dir migrations/$dbtype \
  --config-file diesel-$dbtype.toml

echo "migration $cmd, db: $dbtype - Ok!"