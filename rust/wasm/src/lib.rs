#![forbid(unsafe_code)]

use wasm_bindgen::prelude::*;

use seedfaker_core::eval;
use seedfaker_core::field;
use seedfaker_core::opts;
use seedfaker_core::pipeline;

#[wasm_bindgen]
pub struct SeedFaker {
    master_seed: u64,
    locales: Vec<&'static seedfaker_core::locale::Locale>,
    record_counter: u64,
    tz_offset_minutes: i32,
    since: i64,
    until: i64,
}

#[wasm_bindgen]
impl SeedFaker {
    #[wasm_bindgen(constructor)]
    pub fn new(opts_val: JsValue) -> Result<SeedFaker, JsError> {
        let seed = js_sys::Reflect::get(&opts_val, &"seed".into()).ok().and_then(|v| v.as_string());
        let locale =
            js_sys::Reflect::get(&opts_val, &"locale".into()).ok().and_then(|v| v.as_string());
        let tz = js_sys::Reflect::get(&opts_val, &"tz".into()).ok().and_then(|v| v.as_string());
        let since = js_sys::Reflect::get(&opts_val, &"since".into()).ok().and_then(|v| v.as_f64());
        let until = js_sys::Reflect::get(&opts_val, &"until".into()).ok().and_then(|v| v.as_f64());

        // std::time::SystemTime::now() panics on wasm32-unknown-unknown; derive current
        // epoch from JS when until is not provided.
        let until_secs =
            until.map_or_else(|| (js_sys::Date::now() / 1000.0) as i64, |v| v.trunc() as i64);
        let (master_seed, locales, tz_offset_minutes, since_e, until_e) = opts::resolve_all(
            seed.as_deref(),
            locale.as_deref(),
            tz.as_deref(),
            since.map(|v| format!("{}", v.trunc() as i64)).as_deref(),
            Some(&until_secs.to_string()),
        )
        .map_err(|e| JsError::new(&e))?;
        Ok(Self {
            master_seed,
            locales,
            record_counter: 0,
            tz_offset_minutes,
            since: since_e,
            until: until_e,
        })
    }

    pub fn field(&mut self, spec: &str) -> Result<String, JsError> {
        let (name, mod_str, transform, range_spec, _, omit_pct, _) =
            field::parse_field_spec(spec).map_err(|e| JsError::new(&e))?;
        let f =
            field::lookup(name).ok_or_else(|| JsError::new(&format!("unknown field: {name}")))?;
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

    pub fn record(
        &mut self,
        fields: Vec<String>,
        ctx: Option<String>,
        corrupt: Option<String>,
    ) -> Result<JsValue, JsError> {
        let records = self.records(fields, 1, ctx, corrupt)?;
        let arr: js_sys::Array = records.dyn_into().map_err(|_| JsError::new("internal error"))?;
        Ok(arr.get(0))
    }

    pub fn records(
        &mut self,
        fields: Vec<String>,
        n: u32,
        ctx: Option<String>,
        corrupt: Option<String>,
    ) -> Result<JsValue, JsError> {
        let rec_opts = pipeline::RecordOpts {
            master_seed: self.master_seed,
            locales: &self.locales,
            ctx: opts::resolve_ctx(ctx.as_deref()).map_err(|e| JsError::new(&e))?,
            corrupt_rate: opts::resolve_corrupt_rate(corrupt.as_deref())
                .map_err(|e| JsError::new(&e))?,
            tz_offset_minutes: self.tz_offset_minutes,
            since: self.since,
            until: self.until,
        };
        let count = u64::from(n);
        let (col_names, raw_records) =
            eval::generate_records_from_specs(&fields, &rec_opts, count, self.record_counter)
                .map_err(|e| JsError::new(&e))?;
        self.record_counter += count;

        let result = js_sys::Array::new();
        for vals in &raw_records {
            let obj = js_sys::Object::new();
            for (i, val) in vals.iter().enumerate() {
                js_sys::Reflect::set(
                    &obj,
                    &JsValue::from_str(&col_names[i]),
                    &JsValue::from_str(val),
                )
                .map_err(|_| JsError::new("failed to set property"))?;
            }
            result.push(&obj);
        }
        Ok(result.into())
    }

    pub fn validate(
        fields: Vec<String>,
        ctx: Option<String>,
        corrupt: Option<String>,
    ) -> Result<(), JsError> {
        seedfaker_core::pipeline::validate(&fields, ctx.as_deref(), corrupt.as_deref())
            .map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "fields")]
    pub fn fields_list() -> Vec<String> {
        field::all_names().into_iter().map(String::from).collect()
    }

    pub fn fingerprint() -> String {
        seedfaker_core::fingerprint()
    }

    pub fn build_info() -> String {
        seedfaker_core::build_info()
    }
}
