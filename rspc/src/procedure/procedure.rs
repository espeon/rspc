use std::{fmt, marker::PhantomData};

use super::{builder::GG, ProcedureBuilder};

/// TODO
pub struct Procedure<TCtx = ()> {
    phantom: PhantomData<TCtx>,
}

impl<TCtx> fmt::Debug for Procedure<TCtx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Procedure").finish()
    }
}

impl<TCtx> Procedure<TCtx> {
    /// TODO
    pub fn builder<R, I>() -> ProcedureBuilder<TCtx, GG<R, I>> {
        ProcedureBuilder {
            phantom: PhantomData,
        }
    }

    // TODO: Export types
    // TODO: Run this procedure
}