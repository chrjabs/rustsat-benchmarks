#ifndef C_SOLVER_H
#define C_SOLVER_H

#include "open-wbo.h"

#include "core/SolverTypes.h"
#include "mtl/Vec.h"

class Solver {
  uint32_t *n_vars;
  CollectClauses collector;
  void *collector_data;

public:
  Solver(uint32_t *n_vars, CollectClauses collector, void *collector_data)
      : n_vars(n_vars), collector(collector), collector_data(collector_data) {}

  int nVars() const { return *n_vars; }
  int newVar() { return (*n_vars)++; }
  bool addClause(const NSPACE::vec<NSPACE::Lit> &clause) {
    for (int i = 0; i < clause.size(); i++) {
      struct CLit lit{.x = clause[i].x};
      collector(lit, collector_data);
    }
    collector(clit_Undef, collector_data);
    return true;
  };
};

#endif
