#!/bin/bash
set -exuo pipefail

codegen="$1"
build="$2"

rm -f /tmp/{code,code.[cso],bin,output.txt}

sh -c "$codegen"
wc /tmp/code.[cs]
sh -c "$build"
/tmp/bin <<< '1 2' > /tmp/output.txt
grep -q 3 /tmp/output.txt || {
    echo "Expected 3, but got:"
    cat /tmp/output.txt
    exit 1
}
