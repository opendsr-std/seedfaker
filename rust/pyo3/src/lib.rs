#![forbid(unsafe_code)]

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};
use seedfaker_core::eval;
use seedfaker_core::field;
use seedfaker_core::opts;
use seedfaker_core::pipeline;

fn py_err(e: impl std::fmt::Display) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
}

#[pyclass]
struct SeedFaker {
    master_seed: u64,
    locales: Vec<&'static seedfaker_core::locale::Locale>,
    record_counter: u64,
    tz_offset_minutes: i32,
    since: i64,
    until: i64,
}

#[pymethods]
impl SeedFaker {
    #[new]
    #[pyo3(signature = (seed=None, locale=None, tz=None, since=None, until=None))]
    fn new(
        seed: Option<&str>,
        locale: Option<&str>,
        tz: Option<&str>,
        since: Option<i64>,
        until: Option<i64>,
    ) -> PyResult<Self> {
        let (master_seed, locales, tz_offset_minutes, since_e, until_e) = opts::resolve_all(
            seed,
            locale,
            tz,
            since.map(|v| v.to_string()).as_deref(),
            until.map(|v| v.to_string()).as_deref(),
        )
        .map_err(py_err)?;
        Ok(Self {
            master_seed,
            locales,
            record_counter: 0,
            tz_offset_minutes,
            since: since_e,
            until: until_e,
        })
    }

    fn field(&mut self, spec: &str) -> PyResult<String> {
        let (name, mod_str, transform, range_spec, _, omit_pct, _) =
            field::parse_field_spec(spec).map_err(py_err)?;
        let f = field::lookup(name).ok_or_else(|| py_err(format!("unknown field: {name}")))?;
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

    #[staticmethod]
    #[pyo3(signature = (fields, ctx=None, corrupt=None))]
    fn validate(fields: Vec<String>, ctx: Option<&str>, corrupt: Option<&str>) -> PyResult<()> {
        seedfaker_core::pipeline::validate(&fields, ctx, corrupt).map_err(py_err)
    }

    #[pyo3(signature = (fields, ctx=None, corrupt=None))]
    fn record<'py>(
        &mut self,
        py: Python<'py>,
        fields: Vec<String>,
        ctx: Option<&str>,
        corrupt: Option<&str>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let rec_opts = pipeline::RecordOpts {
            master_seed: self.master_seed,
            locales: &self.locales,
            ctx: opts::resolve_ctx(ctx).map_err(py_err)?,
            corrupt_rate: opts::resolve_corrupt_rate(corrupt).map_err(py_err)?,
            tz_offset_minutes: self.tz_offset_minutes,
            since: self.since,
            until: self.until,
        };
        let (col_names, raw_records) =
            eval::generate_records_from_specs(&fields, &rec_opts, 1, self.record_counter)
                .map_err(py_err)?;
        self.record_counter += 1;

        let dict = PyDict::new(py);
        if let Some(vals) = raw_records.first() {
            for (i, val) in vals.iter().enumerate() {
                dict.set_item(&col_names[i], val)?;
            }
        }
        Ok(dict)
    }

    #[pyo3(signature = (fields, n=1, ctx=None, corrupt=None))]
    fn records<'py>(
        &mut self,
        py: Python<'py>,
        fields: Vec<String>,
        n: usize,
        ctx: Option<&str>,
        corrupt: Option<&str>,
    ) -> PyResult<Bound<'py, PyList>> {
        let rec_opts = pipeline::RecordOpts {
            master_seed: self.master_seed,
            locales: &self.locales,
            ctx: opts::resolve_ctx(ctx).map_err(py_err)?,
            corrupt_rate: opts::resolve_corrupt_rate(corrupt).map_err(py_err)?,
            tz_offset_minutes: self.tz_offset_minutes,
            since: self.since,
            until: self.until,
        };
        let (col_names, raw_records) =
            eval::generate_records_from_specs(&fields, &rec_opts, n as u64, self.record_counter)
                .map_err(py_err)?;
        self.record_counter += n as u64;

        let py_keys: Vec<Bound<'_, PyString>> =
            col_names.iter().map(|k| PyString::new(py, k)).collect();
        let records = PyList::empty(py);
        for vals in &raw_records {
            let dict = PyDict::new(py);
            for (i, val) in vals.iter().enumerate() {
                dict.set_item(&py_keys[i], val)?;
            }
            records.append(dict)?;
        }
        Ok(records)
    }

    #[staticmethod]
    fn fields() -> Vec<&'static str> {
        field::all_names()
    }

    #[staticmethod]
    fn fingerprint() -> String {
        seedfaker_core::fingerprint()
    }

    #[staticmethod]
    fn build_info() -> String {
        seedfaker_core::build_info()
    }
}

#[pymodule]
fn _seedfaker(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SeedFaker>()?;
    Ok(())
}
