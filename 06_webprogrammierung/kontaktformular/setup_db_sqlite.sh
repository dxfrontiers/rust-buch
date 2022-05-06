#!/bin/bash
echo DATABASE_URL=db.sqlite > .env

cargo install diesel_cli --no-default-features --features "sqlite"
diesel setup
diesel migration run

