#!/bin/sh

addr=127.0.0.1
port=7980
docd=./target/

python3 \
	-m http.server \
	--bind "${addr}" \
	--directory "${docd}" \
	${port}
