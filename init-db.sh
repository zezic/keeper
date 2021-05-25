#!/bin/bash
set -e

clickhouse client -u keeper --password 12345 -n <<-EOSQL
	CREATE DATABASE IF NOT EXISTS keeper;
	CREATE TABLE IF NOT EXISTS keeper.entries (
		timestamp DateTime64(6),
		level Enum('DEBUG' = 1, 'INFO' = 2, 'WARNING' = 3, 'ERROR' = 4),
		message String
	) ENGINE = Log;
EOSQL