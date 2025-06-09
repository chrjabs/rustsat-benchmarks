#include <iostream>
#include <string>
#include <zlib.h>

#include "minisat/core/Dimacs.h"
#include "minisat/core/Solver.h"

using namespace Minisat;

int main(int argc, char **argv) {
  Solver solver;
  gzFile in = gzopen(argv[2], "rb");
  parse_DIMACS(in, solver, true);

  int limit = std::stoi(argv[1]);
  vec<Lit> blockClause;
  blockClause.capacity(solver.nVars());

  for (int i = 0; i < limit; i++) {
    bool ret = solver.solve();
    if (!ret) {
      std::cout << "s UNSATISFIABLE" << std::endl;
      break;
    }
    std::cout << "v ";
    for (Var v = 0; v < solver.nVars(); v++) {
      std::cout << ((solver.modelValue(v) == l_True) ? "1" : "0");
      blockClause.push(mkLit(v, solver.modelValue(v) == l_True));
    }
    std::cout << std::endl;
    solver.addClause(blockClause);
    blockClause.clear();
  }
}
