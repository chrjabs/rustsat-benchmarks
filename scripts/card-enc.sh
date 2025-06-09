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

runs=("1 300" "10 300" "20 300" "30 300" "40 300" "50 300" "60 300" "70 300"
"80 300" "90 300" "100 300" "110 300" "120 300" "130 300" "140 300" "150 300"
"160 300" "170 300" "180 300" "190 300" "200 300" "210 300" "220 300" "230 300"
"240 300" "250 300" "260 300" "270 300" "280 300" "290 300" "299 300")

for run in "${runs[@]}"; do
  bound=$(echo ${run} | cut -d ' ' -f1)
  inputs=$(echo ${run} | cut -d ' ' -f2)

  # RustSAT runs
  encodings=("totalizer")
  for enc in "${encodings[@]}"; do
    OUTFILE="bench-output/card-rustsat-${enc}-${bound}-${inputs}.out"
    TIMEFILE="bench-output/card-rustsat-${enc}-${bound}-${inputs}.time"
    { time -p "${RUSTSAT}" card-encoding -E "${enc}" -b "${bound}" -n "${inputs}" > "${OUTFILE}" ; } 2> "${TIMEFILE}"
    EVALFILE="bench-output/card-rustsat-${enc}-${bound}-${inputs}.eval"
    cat "${OUTFILE}" | "${RUSTSAT}" -s "${SEED}" -e card-encoding -b "${bound}" -n "${inputs}" > "${EVALFILE}"
    if grep -q "FAILED" "${EVALFILE}"; then
      >&2 echo "found incorrect encoding for rustsat: ${enc} ${bound} ${inputs}"
    fi
  done

  # PySat runs
  encodings=("totalizer" "seqcounter" "sortnetwrk" "cardnetwrk" "mtotalizer")
  for enc in "${encodings[@]}"; do
    OUTFILE="bench-output/card-pysat-${enc}-${bound}-${inputs}.out"
    TIMEFILE="bench-output/card-pysat-${enc}-${bound}-${inputs}.time"
    { time -p "${PYSAT}" card-encoding -E "${enc}" -b "${bound}" -n "${inputs}" > "${OUTFILE}" ; } 2> "${TIMEFILE}"
    EVALFILE="bench-output/card-pysat-${enc}-${bound}-${inputs}.eval"
    cat "${OUTFILE}" | tr -d '+' | "${RUSTSAT}" -s "${SEED}" -e card-encoding -b "${bound}" -n "${inputs}" > "${EVALFILE}"
    if grep -q "FAILED" "${EVALFILE}"; then
      >&2 echo "found incorrect encoding for pysat: ${enc} ${bound} ${inputs}"
    fi
  done

  # Open-WBO runs
  encodings=("totalizer")
  for enc in "${encodings[@]}"; do
    OUTFILE="bench-output/card-open-wbo-${enc}-${bound}-${inputs}.out"
    TIMEFILE="bench-output/card-open-wbo-${enc}-${bound}-${inputs}.time"
    { time -p "${RUSTSAT}" card-encoding -E "open-wbo-${enc}" -b "${bound}" -n "${inputs}" > "${OUTFILE}" ; } 2> "${TIMEFILE}"
    EVALFILE="bench-output/card-open-wbo-${enc}-${bound}-${inputs}.eval"
    cat "${OUTFILE}" | "${RUSTSAT}" -s "${SEED}" -e card-encoding -b "${bound}" -n "${inputs}" > "${EVALFILE}"
    if grep -q "FAILED" "${EVALFILE}"; then
      >&2 echo "found incorrect encoding for open-wbo: ${enc} ${bound} ${inputs}"
    fi
  done
done
