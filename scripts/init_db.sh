#!/usr/bin/env bash
set -x
set -eo pipefail

#Check if a custom database user has been set, if not use the default
DB_USER=${POSTGRES_USER:=postgres}
#Check if a custom database password has been set, if not use the default
DB_PASS=${POSTGRES_PASSWORD:=password}
#Check if a custom database name has been set, if not use the default
DB_NAME=${POSTGRES_DB:=newsletter}
#Check if a custom database port has been set, if not use the default
DB_PORT=${POSTGRES_PORT:=5432}

#Launch the database
echo "Starting the database"
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASS} \
  -e POSTGRES_DB=${DB_NAME} \
  -p ${DB_PORT}:5432 \
  -d postgres \
  postgres -N 1000

  #TODO add sqlx startup script