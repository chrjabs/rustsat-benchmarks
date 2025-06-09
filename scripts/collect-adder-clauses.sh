#!/usr/bin/env bash

runs=("300 100" "300 200" "300 300" "300 400" "300 500" "300 600")

echo "inputs,rustsat,pysat,open-wbo"
for run in "${runs[@]}"; do
  bound=$(echo ${run} | cut -d ' ' -f1)
  inputs=$(echo ${run} | cut -d ' ' -f2)

  rs_eval="bench-output/pb-rustsat-binary-adder-${bound}-${inputs}.eval"
  py_eval="bench-output/pb-pysat-adder-${bound}-${inputs}.eval"
  owbo_eval="bench-output/pb-open-wbo-adder-${bound}-${inputs}.eval"

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

  echo "${inputs},$(grep '^CLAUSES=' "${rs_eval}" | cut -d '=' -f2),$(grep '^CLAUSES=' "${py_eval}" | cut -d '=' -f2),$(grep '^CLAUSES=' "${owbo_eval}" | cut -d '=' -f2)"
done
