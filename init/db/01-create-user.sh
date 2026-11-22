#!/bin/bash
set -euo pipefail

: ${DB_TESTER_PASSWORD:?DB_TESTER_PASSWORD is required}
: ${DB_OPERATOR_PASSWORD:?DB_OPERATOR_PASSWORD is required}
: ${DB_AUTH_OPERATOR_PASSWORD:?DB_AUTH_OPERATOR_PASSWORD is required}
: ${DB_MIGRATOR_PASSWORD:?DB_MIGRATOR_PASSWORD is required}

echo Creating users...

psql -v ON_ERROR_STOP=1 -U ${POSTGRES_USER} -d ${POSTGRES_USER} <<'QUERY'
  DO $$
  BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'tester') THEN
      CREATE ROLE tester LOGIN;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'migrator') THEN
      CREATE ROLE migrator LOGIN;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'operator') THEN
      CREATE ROLE operator LOGIN;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'auth_operator') THEN
      CREATE ROLE auth_operator LOGIN;
    END IF;
  END $$;
QUERY

echo Setting password...

psql -v ON_ERROR_STOP=1 -U ${POSTGRES_USER} -d ${POSTGRES_USER} \
  -v tester_password=${DB_TESTER_PASSWORD} \
  -v operator_password=${DB_OPERATOR_PASSWORD} \
  -v auth_operator_password=${DB_AUTH_OPERATOR_PASSWORD} \
  -v migrator_password=${DB_MIGRATOR_PASSWORD} <<'QUERY'
  ALTER ROLE tester PASSWORD :'tester_password';
  ALTER ROLE operator PASSWORD :'operator_password';
  ALTER ROLE auth_operator PASSWORD :'auth_operator_password';
  ALTER ROLE migrator PASSWORD :'migrator_password';
QUERY
