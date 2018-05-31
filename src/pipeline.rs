//! # Pipeline
//!
//! Defines an upgrade pipeline that can be used to upgrade and gitify single
//! WordPress plugins.

use wordpress::Plugin;

/// Data for an upgrade pipeline.
pub struct Pipeline {
    plugin: Plugin,
    has_backup: bool,
}

/// Pipeline implementation.
impl Pipeline {
    /// Create a new pipeline instance.
    pub fn new(plugin: Plugin) -> Pipeline {
        Pipeline {
            plugin: plugin,
            has_backup: false,
        }
    }

    pub fn run(self) -> Result<bool, &'static str> {
        Err("Not implemented")
    }
}

pub fn get_pipeline_for_plugin(plugin: Plugin) -> Pipeline {
    let pipeline = Pipeline::new(plugin);

    pipeline
}
