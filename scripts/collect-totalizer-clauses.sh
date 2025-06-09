#!/usr/bin/env bash

runs=("1 300" "10 300" "20 300" "30 300" "40 300" "50 300" "60 300" "70 300"
"80 300" "90 300" "100 300" "110 300" "120 300" "130 300" "140 300" "150 300"
"160 300" "170 300" "180 300" "190 300" "200 300" "210 300" "220 300" "230 300"
"240 300" "250 300" "260 300" "270 300" "280 300" "290 300" "299 300")

echo "bound,rustsat,pysat,open-wbo"
for run in "${runs[@]}"; do
  bound=$(echo ${run} | cut -d ' ' -f1)
  inputs=$(echo ${run} | cut -d ' ' -f2)

  rs_eval="bench-output/card-rustsat-totalizer-${bound}-${inputs}.eval"
  py_eval="bench-output/card-pysat-totalizer-${bound}-${inputs}.eval"
  owbo_eval="bench-output/card-open-wbo-totalizer-${bound}-${inputs}.eval"

  if [[ ! -f "${rs_eval}" ]]; then
    >&2 echo "${rs_eval} does not exist"
    exit 1
  fi
  if [[ ! -f "${py_eval}" ]]; then
    >&2 echo "${py_eval} does not exist"
    exit 1
  fi
  if [[ ! -f "${owbo_eval}" ]]; then
    >&2 echo "${owbo_eval} does not exist"
    exit 1
  fi

  echo "${bound},$(grep '^CLAUSES=' "${rs_eval}" | cut -d '=' -f2),$(grep '^CLAUSES=' "${py_eval}" | cut -d '=' -f2),$(grep '^CLAUSES=' "${owbo_eval}" | cut -d '=' -f2)"
done
