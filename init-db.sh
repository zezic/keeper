#!/bin/bash
set -e

clickhouse client -u keeper --password 12345 -n <<-EOSQL
	CREATE DATABASE keeper;
	CREATE TABLE keeper.entries (level String, message String) ENGINE = Log;
EOSQL