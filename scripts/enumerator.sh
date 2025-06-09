#!/usr/bin/env bash

set -e

mkdir -p bench-output/

RUSTSAT="target/release/enumerator"
CPP="target/cpp-enumerator"

if [[ ! -x "${RUSTSAT}" ]]; then
  >&2 echo "${RUSTSAT} not executable"
  exit 1
fi

INSTANCE="${PWD}/rustsat/data/AProVE11-12.cnf"

limits=(1 10 100 250 500 750 1000)

echo "nsols,rustsat,pysat,cpp"

for limit in "${limits[@]}"; do
  # RustSAT run
  OUTFILE="bench-output/enumerator-rustsat-${limit}.out"
  TIMEFILE="bench-output/enumerator-rustsat-${limit}.time"
  { time -p "${RUSTSAT}" -e ${limit} "${INSTANCE}" > "${OUTFILE}" ; } 2> "${TIMEFILE}"
  UTIME=$(sed -nE 's/user ([0-9]+\.[0-9]+)/\1/p' "${TIMEFILE}")
  STIME=$(sed -nE 's/sys ([0-9]+\.[0-9]+)/\1/p' "${TIMEFILE}")
  RUST_TIME=$(echo "${UTIME} ${STIME}" | awk '{print $1 + $2}')

  # PySat run
  OUTFILE="bench-output/enumerator-pysat-${limit}.out"
  TIMEFILE="bench-output/enumerator-pysat-${limit}.time"
  { time -p python -m "pysat.examples.models" -e "${limit}" "${INSTANCE}" > "${OUTFILE}" ; } 2> "${TIMEFILE}"
  UTIME=$(sed -nE 's/user ([0-9]+\.[0-9]+)/\1/p' "${TIMEFILE}")
  STIME=$(sed -nE 's/sys ([0-9]+\.[0-9]+)/\1/p' "${TIMEFILE}")
  PY_TIME=$(echo "${UTIME} ${STIME}" | awk '{print $1 + $2}')

  # C++ run
  OUTFILE="bench-output/enumerator-cpp-${limit}.out"
  TIMEFILE="bench-output/enumerator-cpp-${limit}.time"
  { time -p "${CPP}" ${limit} "${INSTANCE}" > "${OUTFILE}" ; } 2> "${TIMEFILE}"
  UTIME=$(sed -nE 's/user ([0-9]+\.[0-9]+)/\1/p' "${TIMEFILE}")
  STIME=$(sed -nE 's/sys ([0-9]+\.[0-9]+)/\1/p' "${TIMEFILE}")
  CPP_TIME=$(echo "${UTIME} ${STIME}" | awk '{print $1 + $2}')

  echo "${limit},${RUST_TIME},${PY_TIME},${CPP_TIME}"
done
