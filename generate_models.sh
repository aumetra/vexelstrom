#!/usr/bin/env bash

set -o errexit -o nounset -o pipefail

diesel_ext -t -d "diesel::Identifiable, diesel::Selectable, diesel::Queryable" -I "super::schema::*" -I "time::OffsetDateTime" --map "Timestamptz OffsetDateTime" > src/db/models.rs
