#!/usr/bin/env bash
$PWD/tools/bin/jj config set --user user.name  "$(git config --system user.name)"
$PWD/tools/bin/jj config set --user user.email "$(git config --system user.email)"
$PWD/tools/bin/jj git init --colocate || true
$PWD/tools/bin/jj branch track main@origin
