#!/bin/sh
CONVIS_ARGS=${CONVIS_ARGS:-"${@:2}"}
ulimit -l 81920000
mount -t debugfs debugfs /sys/kernel/debug
convis -v ${CONVIS_ARGS}
