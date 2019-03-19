#!/bin/sh

. .env

dbtype=$1

if [ $dbtype = "read" ]
then
  connection=$DATABASE_URL_READ
else
  connection=$DATABASE_URL_WRITE
fi

psql "$connection"