use indexmap::IndexMap;
use itertools::Itertools;

use crate::cli::args::runtime::RuntimeArg;
use crate::config::Config;
use crate::env;
use crate::toolset::tool_version::ToolVersionType;
use crate::toolset::{ToolSource, ToolVersion, Toolset};

#[derive(Debug)]
pub struct ToolsetBuilder {
    args: Vec<RuntimeArg>,
    install_missing: bool,
}

impl ToolsetBuilder {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            install_missing: false,
        }
    }

    pub fn with_args(mut self, args: &[RuntimeArg]) -> Self {
        self.args = args.to_vec();
        self
    }

    pub fn with_install_missing(mut self) -> Self {
        self.install_missing = true;
        self
    }

    pub fn build(self, config: &Config) -> Toolset {
        let mut toolset = Toolset::default().with_plugins(config.plugins.clone());
        load_config_files(config, &mut toolset);
        load_runtime_env(&mut toolset, env::vars().collect());
        load_runtime_args(&mut toolset, &self.args);
        toolset.resolve(config);

        if self.install_missing {
            if let Err(e) = toolset.install_missing(config, None) {
                warn!("Error installing runtimes: {}", e);
            };
        }

        debug!("{}", toolset);
        toolset
    }
}

fn load_config_files(config: &Config, ts: &mut Toolset) {
    for cf in config.config_files.values().rev() {
        ts.merge(cf.to_toolset());
    }
}

fn load_runtime_env(ts: &mut Toolset, env: IndexMap<String, String>) {
    for (k, v) in env {
        if k.starts_with("RTX_") && k.ends_with("_VERSION") {
            let plugin_name = k[4..k.len() - 8].to_lowercase();
            let source = ToolSource::Environment(k, v.clone());
            let mut env_ts = Toolset::new(source);
            let version = ToolVersion::new(plugin_name.clone(), ToolVersionType::Version(v));
            env_ts.add_version(plugin_name, version);
            ts.merge(&env_ts);
        }
    }
}

fn load_runtime_args(ts: &mut Toolset, args: &[RuntimeArg]) {
    for (plugin_name, args) in args.iter().into_group_map_by(|arg| arg.plugin.clone()) {
        let mut arg_ts = Toolset::new(ToolSource::Argument);
        for arg in args {
            if let Some(version) = arg.to_tool_version() {
                arg_ts.add_version(plugin_name.clone(), version);
            }
        }
        ts.merge(&arg_ts);
    }
}
