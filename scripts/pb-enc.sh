#!/usr/bin/env bash

set -e

mkdir -p bench-output/

SEED=42

RUSTSAT=target/release/rustsat-benchmarks
PYSAT=pysrc/main.py

if [[ ! -x "${RUSTSAT}" ]]; then
  >&2 echo "${RUSTSAT} not executable"
  exit 1
fi

if [[ ! -x "${PYSAT}" ]]; then
  >&2 echo "${PYSAT} not executable"
  exit 1
fi

runs=("1 300" "50 300" "100 300" "150 300" "200 300" "250 300" "300 300"
"400 300" "500 300" "600 300" "300 100" "300 200" "300 400" "300 500" "300 600")

for run in "${runs[@]}"; do
  bound=$(echo ${run} | cut -d ' ' -f1)
  inputs=$(echo ${run} | cut -d ' ' -f2)
  weights=$("${RUSTSAT}" -s "${SEED}" random-weights -n "${inputs}")

  # RustSAT runs
  encodings=("gte" "binary-adder" "dpw")
  for enc in "${encodings[@]}"; do
    OUTFILE="bench-output/pb-rustsat-${enc}-${bound}-${inputs}.out"
    TIMEFILE="bench-output/pb-rustsat-${enc}-${bound}-${inputs}.time"
    { time -p "${RUSTSAT}" pb-encoding -E "${enc}" -b "${bound}" ${weights} > "${OUTFILE}" ; } 2> "${TIMEFILE}"
    EVALFILE="bench-output/pb-rustsat-${enc}-${bound}-${inputs}.eval"
    cat "${OUTFILE}" | "${RUSTSAT}" -s "${SEED}" -e pb-encoding -b "${bound}" ${weights} > "${EVALFILE}"
    if grep -q "FAILED" "${EVALFILE}"; then
      >&2 echo "found incorrect encoding for rustsat: ${enc} ${bound} ${inputs}"
    fi
  done

  # PySat runs
  encodings=("bdd" "seqcounter" "sortnetwrk" "adder" "binmerge")
  for enc in "${encodings[@]}"; do
    OUTFILE="bench-output/pb-pysat-${enc}-${bound}-${inputs}.out"
    TIMEFILE="bench-output/pb-pysat-${enc}-${bound}-${inputs}.time"
    { time -p "${PYSAT}" pb-encoding -E "${enc}" -b "${bound}" ${weights} > "${OUTFILE}" ; } 2> "${TIMEFILE}"
    EVALFILE="bench-output/pb-pysat-${enc}-${bound}-${inputs}.eval"
    cat "${OUTFILE}" | tr -d '+' | "${RUSTSAT}" -s "${SEED}" -e pb-encoding -b "${bound}" ${weights} > "${EVALFILE}"
    if grep -q "FAILED" "${EVALFILE}"; then
      >&2 echo "found incorrect encoding for pysat: ${enc} ${bound} ${inputs}"
    fi
  done

  # Open-WBO runs
  encodings=("gte" "adder")
  for enc in "${encodings[@]}"; do
    OUTFILE="bench-output/pb-open-wbo-${enc}-${bound}-${inputs}.out"
    TIMEFILE="bench-output/pb-open-wbo-${enc}-${bound}-${inputs}.time"
    { time -p "${RUSTSAT}" pb-encoding -E "open-wbo-${enc}" -b "${bound}" ${weights} > "${OUTFILE}" ; } 2> "${TIMEFILE}"
    EVALFILE="bench-output/pb-open-wbo-${enc}-${bound}-${inputs}.eval"
    cat "${OUTFILE}" | "${RUSTSAT}" -s "${SEED}" -e pb-encoding -b "${bound}" ${weights} > "${EVALFILE}"
    if grep -q "FAILED" "${EVALFILE}"; then
      >&2 echo "found incorrect encoding for open-wbo: ${enc} ${bound} ${inputs}"
    fi
  done
done
