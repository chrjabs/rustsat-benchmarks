#!/usr/bin/env python3

import argparse
from pysat import card
from pysat import pb


def card_enc(enc: card.EncType, n_inputs: int, bound: int):
    cnf = card.CardEnc.atmost(lits=range(1,n_inputs+1), bound=bound, encoding=enc)
    print(cnf.to_dimacs())


def card_enc_main(args):
    if args.encoding == "totalizer":
        enc = card.EncType.totalizer
    elif args.encoding == "seqcounter":
        enc = card.EncType.seqcounter
    elif args.encoding == "sortnetwrk":
        enc = card.EncType.sortnetwrk
    elif args.encoding == "cardnetwrk":
        enc = card.EncType.cardnetwrk
    elif args.encoding == "mtotalizer":
        enc = card.EncType.mtotalizer
    card_enc(enc, args.n_inputs, args.bound)


def pb_enc(enc: pb.EncType, weights: list[int], bound: int):
    cnf = pb.PBEnc.atmost(lits=list(range(1,len(weights)+1)), weights=weights, bound=bound, encoding=enc)
    print(cnf.to_dimacs())


def pb_enc_main(args):
    if args.encoding == "bdd":
        enc = pb.EncType.bdd
    elif args.encoding == "seqcounter":
        enc = pb.EncType.seqcounter
    elif args.encoding == "sortnetwrk":
        enc = pb.EncType.sortnetwrk
    elif args.encoding == "adder":
        enc = pb.EncType.adder
    elif args.encoding == "binmerge":
        enc = pb.EncType.binmerge
    pb_enc(enc, args.weights, args.bound)


def main():
    parser = argparse.ArgumentParser(
        prog="pysat-benchmarks",
        description="benchmarks for PySat to compare to RustSAT",
    )

    subparsers = parser.add_subparsers(help = "the benchmark to run")

    card_enc = subparsers.add_parser("card-encoding", help = "cardinality encoding benchmark")
    card_enc.set_defaults(func=card_enc_main)
    card_enc.add_argument("-E", "--encoding",
                          choices=("totalizer", "seqcounter", "sortnetwrk", "cardnetwrk", "mtotalizer"),
                          help="the cardinality encoding to use")
    card_enc.add_argument("-b", "--bound", type=int, default=150,
                          help="the cardinality bound to encode on the input literals")
    card_enc.add_argument("-n", "--n-inputs", type=int, default=300,
                          help="the number of inputs to generate the cardinality encoding for")

    pb_enc = subparsers.add_parser("pb-encoding", help = "pb encoding benchmark")
    pb_enc.set_defaults(func=pb_enc_main)
    pb_enc.add_argument("-E" , "--encoding",
                        choices=("bdd", "seqcounter", "sortnetwrk", "adder", "binmerge"),
                        help="the pb encoding to use")
    pb_enc.add_argument("-b", "--bound", type=int, default=150,
                        help="the cardinality bound to encode on the input literals")
    pb_enc.add_argument("weights", type=int, action="extend", nargs="*",
                        help="the weights for the input literals")

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
