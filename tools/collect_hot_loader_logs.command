#!/bin/zsh
set -u

echo "----- ELASTICGRID SHELL -----"
cat /tmp/ae-hot-loader-elasticgrid-shell.log 2>/dev/null || echo "(no ElasticGrid shell log)"
echo
echo "----- AE HOT LOADER SHELL RELOADER -----"
cat /tmp/ae-hot-loader-shell-reloader.log 2>/dev/null || echo "(no shell reloader log)"
echo
echo "----- AE HOT LOADER AGENT -----"
cat /tmp/ae-hot-loader-agent.log 2>/dev/null || echo "(no agent log)"
