#!/bin/bash

args="$*"
filtered_args=${args/ERROR_ON_UNDEFINED_SYMBOLS\=1/ERROR_ON_UNDEFINED_SYMBOLS\=0}
emcc $filtered_args -s NO_EXIT_RUNTIME=1 -s NO_FILESYSTEM=1 -s USE_PTHREADS=0 -s LEGALIZE_JS_FFI=0
