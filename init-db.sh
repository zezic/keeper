#!/bin/bash
set -e

clickhouse client -u $CLICKHOUSE_USER --password $CLICKHOUSE_PASSWORD -n <<-EOSQL
	CREATE DATABASE IF NOT EXISTS $CLICKHOUSE_DATABASE;
	CREATE TABLE IF NOT EXISTS $CLICKHOUSE_DATABASE.entries (
		timestamp DateTime64(9),
		level Enum8('DEBUG' = 1, 'INFO' = 2, 'WARNING' = 3, 'ERROR' = 4),
		message String
	)
	ENGINE = MergeTree
	ORDER BY timestamp;
EOSQL