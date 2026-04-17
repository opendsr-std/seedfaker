#![allow(unsafe_code)]

use napi_derive::napi;
use seedfaker_core::eval;
use seedfaker_core::field;
use seedfaker_core::opts;
use seedfaker_core::pipeline;
use std::collections::HashMap;

fn napi_err(e: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(e.to_string())
}

#[napi]
pub struct SeedFaker {
    master_seed: u64,
    locales: Vec<&'static seedfaker_core::locale::Locale>,
    record_counter: u64,
    tz_offset_minutes: i32,
    since: i64,
    until: i64,
}

#[napi]
impl SeedFaker {
    #[napi(constructor)]
    pub fn new(
        seed: Option<String>,
        locale: Option<String>,
        tz: Option<String>,
        since: Option<i64>,
        until: Option<i64>,
    ) -> napi::Result<Self> {
        let (master_seed, locales, tz_offset_minutes, since_e, until_e) = opts::resolve_all(
            seed.as_deref(),
            locale.as_deref(),
            tz.as_deref(),
            since.map(|v| v.to_string()).as_deref(),
            until.map(|v| v.to_string()).as_deref(),
        )
        .map_err(napi_err)?;
        Ok(Self {
            master_seed,
            locales,
            record_counter: 0,
            tz_offset_minutes,
            since: since_e,
            until: until_e,
        })
    }

    #[napi]
    pub fn field(&mut self, spec: String) -> napi::Result<String> {
        let (name, mod_str, transform, range_spec, _, omit_pct, _) =
            field::parse_field_spec(&spec).map_err(napi_err)?;
        let f = field::lookup(name).ok_or_else(|| napi_err(format!("unknown field: {name}")))?;
        let range = field::resolve_range(&range_spec, f.name, self.since, self.until);
        let dh = pipeline::field_domain_hash(self.master_seed, f, mod_str);

        let fs = pipeline::FieldSpec {
            field: f,
            modifier: mod_str,
            domain_hash: dh,
            range,
            transform,
            omit_pct,
        };
        let mut vals = pipeline::generate_field_values(
            &fs,
            1,
            &mut self.record_counter,
            &self.locales,
            self.tz_offset_minutes,
            self.since,
            self.until,
        );
        Ok(vals.pop().unwrap_or_default())
    }

    #[napi]
    pub fn validate(
        &self,
        fields: Vec<String>,
        ctx: Option<String>,
        corrupt: Option<String>,
    ) -> napi::Result<()> {
        seedfaker_core::pipeline::validate(&fields, ctx.as_deref(), corrupt.as_deref())
            .map_err(napi_err)
    }

    #[napi]
    pub fn record(
        &mut self,
        fields: Vec<String>,
        ctx: Option<String>,
        corrupt: Option<String>,
    ) -> napi::Result<HashMap<String, String>> {
        let mut result = self.records(fields, Some(1), ctx, corrupt)?;
        Ok(result.swap_remove(0))
    }

    #[napi]
    pub fn records(
        &mut self,
        fields: Vec<String>,
        n: Option<u32>,
        ctx: Option<String>,
        corrupt: Option<String>,
    ) -> napi::Result<Vec<HashMap<String, String>>> {
        let rec_opts = pipeline::RecordOpts {
            master_seed: self.master_seed,
            locales: &self.locales,
            ctx: opts::resolve_ctx(ctx.as_deref()).map_err(napi_err)?,
            corrupt_rate: opts::resolve_corrupt_rate(corrupt.as_deref()).map_err(napi_err)?,
            tz_offset_minutes: self.tz_offset_minutes,
            since: self.since,
            until: self.until,
        };
        let count = u64::from(n.unwrap_or(1));
        let (col_names, raw_records) =
            eval::generate_records_from_specs(&fields, &rec_opts, count, self.record_counter)
                .map_err(napi_err)?;
        self.record_counter += count;

        Ok(raw_records
            .into_iter()
            .map(|vals| {
                let mut record = HashMap::with_capacity(col_names.len());
                for (i, val) in vals.into_iter().enumerate() {
                    record.insert(col_names[i].clone(), val);
                }
                record
            })
            .collect())
    }

    #[napi]
    pub fn fields() -> Vec<&'static str> {
        field::all_names()
    }

    #[napi]
    pub fn fingerprint() -> String {
        seedfaker_core::fingerprint()
    }

    #[napi]
    pub fn build_info() -> String {
        seedfaker_core::build_info()
    }
}
