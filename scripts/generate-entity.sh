#!/usr/bin/env bash
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

sea-orm-cli generate entity -l -o "$ROOT_DIR/apps/task-api/entity/src"
