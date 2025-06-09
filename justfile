benches: card-enc-bench pb-enc-bench enumerator-bench

card-enc-bench: build-rust-bench && (plot "totalizer")
    bash scripts/card-enc.sh
    bash scripts/collect-totalizer-clauses.sh > data/totalizer.csv

pb-enc-bench: build-rust-bench && (plot "gte") (plot "binary-adder")
    bash scripts/pb-enc.sh
    bash scripts/collect-gte-clauses.sh > data/gte.csv
    bash scripts/collect-adder-clauses.sh > data/binary-adder.csv

enumerator-bench: build-rust-enumerator build-cpp-enumerator && (plot "enumerator")
    bash scripts/enumerator.sh > data/enumerator.csv

build-rust-bench:
    cargo build --release

build-rust-enumerator:
    cargo build --manifest-path rustsat/Cargo.toml --target-dir target --release -p rustsat-tools --bin enumerator

build-cpp-enumerator: build-minisat-static
    clang++ -O3 -Irustsat/minisat/cppsrc -c enumerator.cpp -o target/enumerator.o
    clang++ -lz target/enumerator.o target/cpp-minisat/libminisat.a -o target/cpp-enumerator

build-minisat-static:
    mkdir -p target/cpp-minisat
    cmake -S rustsat/minisat/cppsrc -B target/cpp-minisat -DCMAKE_BUILD_TYPE=Release
    cmake --build target/cpp-minisat --target minisat-lib-static

plot name:
    latexmk -cd -pdf plots/{{ name }}.tex
    pdfcrop plots/{{ name }}.pdf
    mv plots/{{ name }}-crop.pdf plots/{{ name }}.pdf
    pdftoppm -singlefile -png -r 300 plots/{{ name }}.pdf plots/{{ name }}
