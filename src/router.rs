use std::{
    collections::BTreeMap,
    fmt,
    fs::{self, File},
    io::Write,
    sync::Arc,
};

use crate::{error::ExportError, export_config::ExportConfig};

use rspc_core::Executor;
use specta::{
    ts::{self},
    TypeMap,
};

use crate::router_builder2::ProcedureMap;

/// Router is a router that has been constructed and validated. It is ready to be attached to an integration to serve it to the outside world!
pub struct Router<TCtx = ()> {
    // TODO: Single map
    pub(crate) queries: ProcedureMap<TCtx>,
    // pub(crate) mutations: ProcedureMap<TCtx>,
    // pub(crate) subscriptions: ProcedureMap<TCtx>,
    pub(crate) typ_store: TypeMap,
}

impl<TCtx> fmt::Debug for Router<TCtx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router").finish()
    }
}

// This is to avoid needing to constrain `TCtx: Default` like the derive macro requires
impl<TCtx> Default for Router<TCtx> {
    fn default() -> Self {
        Self {
            queries: Default::default(),
            // mutations: Default::default(),
            // subscriptions: Default::default(),
            typ_store: Default::default(),
        }
    }
}

impl<TCtx> Router<TCtx>
where
    TCtx: Send + 'static,
{
    // // TODO: Remove this and force it to always be `Arc`ed from the point it was constructed???
    // pub fn arced(self) -> Arc<Self> {
    //     Arc::new(self)
    // }

    #[allow(clippy::panic_in_result_fn)] // TODO: Error handling given we return `Result`
    pub fn export_ts(&self, cfg: ExportConfig) -> Result<(), ExportError> {
        if let Some(export_dir) = cfg.export_path.parent() {
            fs::create_dir_all(export_dir)?;
        }
        let mut file = File::create(&cfg.export_path)?;
        if cfg.header != "" {
            writeln!(file, "{}", cfg.header)?;
        }
        writeln!(file, "// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.")?;

        let config = ts::ExportConfig::new().bigint(
            ts::BigIntExportBehavior::FailWithReason(
                "rspc does not support exporting bigint types (i64, u64, i128, u128) because they are lossily decoded by `JSON.parse` on the frontend. Tracking issue: https://github.com/oscartbeaumont/rspc/issues/93",
            )
        );

        // TODO: Specta API + `ExportConfig` option for a formatter
        todo!();
        // writeln!(
        //     file,
        //     "{}",
        //     ts::export_named_datatype(
        //         &config,
        //         &ProceduresDef::new(
        //             self.queries.values(),
        //             self.mutations.values(),
        //             self.subscriptions.values()
        //         )
        //         .to_named(),
        //         &self.typ_store()
        //     )?
        // )?;

        // We sort by name to detect duplicate types BUT also to ensure the output is deterministic. The SID can change between builds so is not suitable for this.
        let types = self.typ_store.clone();
        let types = types.iter().collect::<BTreeMap<_, _>>();

        // This is a clone of `detect_duplicate_type_names` but using a `BTreeMap` for deterministic ordering
        let mut map = BTreeMap::new();
        for (sid, dt) in &types {
            if let Some(ext) = dt.ext() {
                if let Some((existing_sid, existing_impl_location)) =
                    map.insert(dt.name(), (sid, *ext.impl_location()))
                {
                    if existing_sid != sid {
                        return Err(ExportError::TsExportErr(
                            ts::ExportError::DuplicateTypeName(
                                dt.name().clone(),
                                *ext.impl_location(),
                                existing_impl_location,
                            ),
                        ));
                    }
                }
            }
        }

        // TODO: We can probs avoid doing this
        let mut new_types = TypeMap::default();
        for (sid, dt) in types.iter() {
            new_types.insert(*sid, (*dt).clone());
        }

        for (_, (sid, _)) in map {
            writeln!(
                file,
                "\n{}",
                ts::export_named_datatype(
                    &config,
                    match types.get(sid) {
                        Some(v) => v,
                        _ => unreachable!(),
                    },
                    &new_types
                )?
            )?;
        }

        file.flush()?;
        drop(file);

        if let Some(formatter) = cfg.formatter {
            (formatter)(cfg.export_path)?;
        }

        Ok(())
    }
}

impl<TCtx> rspc_core::internal::SealedRouter for Router<TCtx> {}
impl<TCtx> rspc_core::IntoRouter for Router<TCtx> {
    type Ctx = TCtx;

    fn build(self) -> rspc_core::Executor {
        let mut executor = Executor::new();

        for (name, procedure) in self.queries {
            executor.insert(
                name.into(),
                Arc::new(|ctx| {
                    Box::pin(async move {
                        ctx.result.serialize(&"Hello World");
                    })
                }),
            );
        }

        // TODO: `mutations` and `subscriptions`

        executor
    }
}
