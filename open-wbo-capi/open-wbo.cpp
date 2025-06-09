#include "open-wbo.h"

#include "Solver.h"
#include "encodings/Enc_Adder.h"
#include "encodings/Enc_GTE.h"
#include "encodings/Enc_Totalizer.h"

extern "C" {

const struct CLit clit_Undef{.x = -2};

void open_wbo_totalizer(const struct CLit *lits, uint32_t n_lits, uint64_t rhs,
                        uint32_t *n_vars, CollectClauses collector,
                        void *collector_data) {
  NSPACE::vec<Glucose::Lit> lits_v((const Glucose::Lit *)lits, n_lits);
  Solver solver(n_vars, collector, collector_data);

  openwbo::Totalizer enc;
  enc.build(&solver, lits_v, rhs);
  if (enc.hasCreatedEncoding())
    enc.update(&solver, rhs);
}

void open_wbo_gte(const struct CLit *lits, const uint64_t *coeffs,
                  uint32_t n_lits, uint64_t rhs, uint32_t *n_vars,
                  CollectClauses collector, void *collector_data) {
  NSPACE::vec<Glucose::Lit> lits_v((const Glucose::Lit *)lits, n_lits);
  NSPACE::vec<uint64_t> coeffs_v(coeffs, n_lits);
  Solver solver(n_vars, collector, collector_data);

  openwbo::GTE enc;
  enc.encode(&solver, lits_v, coeffs_v, rhs);
}

void open_wbo_adder(const struct CLit *lits, const uint64_t *coeffs,
                    uint32_t n_lits, uint64_t rhs, uint32_t *n_vars,
                    CollectClauses collector, void *collector_data) {
  NSPACE::vec<Glucose::Lit> lits_v((const Glucose::Lit *)lits, n_lits);
  NSPACE::vec<uint64_t> coeffs_v(coeffs, n_lits);
  Solver solver(n_vars, collector, collector_data);

  openwbo::Adder enc;
  enc.encode(&solver, lits_v, coeffs_v, rhs);
}
}
