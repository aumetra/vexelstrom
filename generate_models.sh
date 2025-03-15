#!/usr/bin/env bash

set -o errexit -o nounset -o pipefail

diesel_ext -I "time::OffsetDateTime" -I "diesel::Queryable" --map "Timestamptz OffsetDateTime" > src/db/models.rs
