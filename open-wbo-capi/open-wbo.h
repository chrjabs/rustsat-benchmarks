#ifndef OPEN_WBO_C_H
#define OPEN_WBO_C_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

struct CLit {
  int x;
};

extern const struct CLit clit_Undef;

typedef void (*CollectClauses)(struct CLit, void *);

void open_wbo_totalizer(const struct CLit *lits, uint32_t n_lits, uint64_t rhs,
                        uint32_t *n_vars, CollectClauses collector,
                        void *collector_data);

void open_wbo_gte(const struct CLit *lits, const uint64_t *coeffs,
                  uint32_t n_lits, uint64_t rhs, uint32_t *n_vars,
                  CollectClauses collector, void *collector_data);

void open_wbo_adder(const struct CLit *lits, const uint64_t *coeffs,
                    uint32_t n_lits, uint64_t rhs, uint32_t *n_vars,
                    CollectClauses collector, void *collector_data);

#ifdef __cplusplus
}
#endif

#endif
