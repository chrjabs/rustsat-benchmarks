use anyhow::Result;
use rustsat::types::Lit;

pub fn totalizer(n_inputs: u32, bound: usize) -> Result<()> {
    let lits: Vec<_> = (0..n_inputs).map(|idx| Lit::new(idx, false)).collect();
    let mut n_vars = n_inputs;
    let mut collector = ffi::Collector::default();
    unsafe {
        ffi::open_wbo_totalizer(
            lits.as_ptr().cast(),
            n_inputs,
            bound as u64,
            &mut n_vars,
            Some(ffi::collect_clauses),
            ((&mut collector) as *mut ffi::Collector).cast(),
        )
    };
    let cnf = collector.as_cnf();
    cnf.write_dimacs(&mut std::io::stdout(), n_vars)?;
    Ok(())
}

pub fn gte(weights: &[usize], bound: usize) -> Result<()> {
    let n_inputs = u32::try_from(weights.len())?;
    let lits: Vec<_> = (0..n_inputs).map(|idx| Lit::new(idx, false)).collect();
    let weights: Vec<_> = weights.iter().map(|w| *w as u64).collect();
    let mut n_vars = n_inputs;
    let mut collector = ffi::Collector::default();
    unsafe {
        ffi::open_wbo_gte(
            lits.as_ptr().cast(),
            weights.as_ptr().cast(),
            n_inputs,
            bound as u64,
            &mut n_vars,
            Some(ffi::collect_clauses),
            ((&mut collector) as *mut ffi::Collector).cast(),
        )
    };
    let cnf = collector.as_cnf();
    cnf.write_dimacs(&mut std::io::stdout(), n_vars)?;
    Ok(())
}

pub fn adder(weights: &[usize], bound: usize) -> Result<()> {
    let n_inputs = u32::try_from(weights.len())?;
    let lits: Vec<_> = (0..n_inputs).map(|idx| Lit::new(idx, false)).collect();
    let weights: Vec<_> = weights.iter().map(|w| *w as u64).collect();
    let mut n_vars = n_inputs;
    let mut collector = ffi::Collector::default();
    unsafe {
        ffi::open_wbo_adder(
            lits.as_ptr().cast(),
            weights.as_ptr().cast(),
            n_inputs,
            bound as u64,
            &mut n_vars,
            Some(ffi::collect_clauses),
            ((&mut collector) as *mut ffi::Collector).cast(),
        )
    };
    let cnf = collector.as_cnf();
    cnf.write_dimacs(&mut std::io::stdout(), n_vars)?;
    Ok(())
}

mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    use std::ffi::c_void;

    use rustsat::{
        instances::Cnf,
        types::{Clause, Lit},
    };

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[derive(Debug, Default)]
    pub struct Collector {
        cnf: Cnf,
        buffer: Clause,
    }

    impl Collector {
        pub fn as_cnf(self) -> Cnf {
            debug_assert!(self.buffer.is_empty());
            self.cnf
        }
    }

    pub unsafe extern "C" fn collect_clauses(lit: CLit, collector: *mut c_void) {
        let collector = collector.cast::<Collector>();
        if lit.x == clit_Undef.x {
            let mut clause = Clause::default();
            std::mem::swap(&mut clause, &mut (*collector).buffer);
            (*collector).cnf.add_clause(clause);
        } else {
            (*collector)
                .buffer
                .add(std::mem::transmute::<CLit, Lit>(lit));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gte_basic() {
        super::gte(&[10, 20, 30], 20).unwrap();
    }
}
