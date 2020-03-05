use std::env;
use std::path::PathBuf;

use once_cell::sync::OnceCell;

use lrpmp_spec::naming::RUST_NAMING_CONVENTION;
use lrpmp_spec::{Error, Spec};

static GLOBAL_SPEC_SOURCE: OnceCell<SpecSource> = OnceCell::new();

struct SpecSource {
    spec: Spec,
    path: Option<PathBuf>,
}

pub fn get_spec(spec_path_opt: Option<String>) -> Result<Spec, Error> {
    let spec_path = get_spec_path(spec_path_opt);
    let global_src = GLOBAL_SPEC_SOURCE.get_or_try_init(|| {
        let path = spec_path.clone();
        load_spec(&path).map(|spec| SpecSource { spec, path })
    })?;
    if spec_path == global_src.path {
        Ok(global_src.spec.clone())
    } else {
        load_spec(&spec_path)
    }
}

fn load_spec(spec_path: &Option<PathBuf>) -> Result<Spec, Error> {
    match spec_path {
        Some(spec_path) => Spec::load(spec_path)?
            .rename(RUST_NAMING_CONVENTION)
            .validate(),
        None => Ok(Spec::current()?.rename(RUST_NAMING_CONVENTION)),
    }
}

fn get_spec_path(spec_path_opt: Option<String>) -> Option<PathBuf> {
    spec_path_opt.map(|spec_path| {
        env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::new())
            .join(spec_path)
    })
}
