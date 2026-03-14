#!/usr/bin/env bash
# Run KwaaiNet Node Dashboard (backend + frontend dev servers)
set -e
cd "$(dirname "$0")"
npm install
npm run dev
