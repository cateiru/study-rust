#!/usr/sh

ARGS=$1

if [ $ARGS ]; then
  echo Build to ${ARGS}
  rustc ${ARGS}
  echo Run to ${ARGS%.*}
  ./${ARGS%.*}
else
  for i in `find . -name "*.rs" -type f`; do
    echo Build to ${i}
    rustc ${i}
  done
fi
