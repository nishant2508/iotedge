// Copyright (c) Microsoft. All rights reserved.

#[derive(Clone, Default, serde::Serialize)]
pub struct Config {}

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub type_: String,
    pub config: Config,
}

impl Default for Module {
    fn default() -> Self {
        Module {
            name: "testModule".to_string(),
            type_: "test".to_string(),
            config: Config::default(),
        }
    }
}

#[async_trait::async_trait]
impl edgelet_core::Module for Module {
    type Config = Config;
    type Error = std::io::Error;

    fn name(&self) -> &str {
        &self.name
    }

    fn type_(&self) -> &str {
        &self.type_
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    // The functions below aren't used in tests.

    async fn runtime_state(&self) -> Result<edgelet_core::ModuleRuntimeState, Self::Error> {
        unimplemented!()
    }
}

pub struct ModuleRegistry {}

#[async_trait::async_trait]
impl edgelet_core::ModuleRegistry for ModuleRegistry {
    type Config = Config;
    type Error = std::io::Error;

    // The fuctions below aren't used in tests.

    async fn pull(&self, _config: &Self::Config) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn remove(&self, _name: &str) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

pub struct Runtime {
    pub module_top_resp: Option<std::collections::BTreeMap<String, Vec<i32>>>,
}

impl Runtime {
    /// Return a generic error. Most users of ModuleRuntime don't act on the error other
    /// than passing it up the call stack, so it's fine to return any error.
    fn test_error() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, "test error")
    }

    pub fn clear_auth(&mut self) {
        // Empty PID array for auth will deny all requests.
        self.module_top_resp = Some(std::collections::BTreeMap::new());
    }
}

impl Default for Runtime {
    fn default() -> Self {
        // The PID in module_top is used for auth. Bypass auth when testing by always placing
        // this process's PID in the default module_top response.
        let pid = nix::unistd::getpid().as_raw();

        let mut modules = std::collections::BTreeMap::new();
        modules.insert("default".to_string(), vec![pid]);

        Runtime {
            module_top_resp: Some(modules),
        }
    }
}

#[async_trait::async_trait]
impl edgelet_core::ModuleRuntime for Runtime {
    type Error = std::io::Error;

    type Config = Config;
    type Module = Module;
    type ModuleRegistry = ModuleRegistry;

    type Chunk = bytes::Bytes;
    type Logs =
        std::pin::Pin<Box<dyn futures::Stream<Item = Result<Self::Chunk, Self::Error>> + Send>>;

    async fn module_top(&self, id: &str) -> Result<Vec<i32>, Self::Error> {
        if let Some(modules) = &self.module_top_resp {
            let pids = if let Some(pids) = modules.get(id) {
                pids.clone()
            } else {
                if let Some(default) = modules.get("default") {
                    default.clone()
                } else {
                    Vec::new()
                }
            };

            Ok(pids)
        } else {
            Err(Self::test_error())
        }
    }

    // The functions below aren't used in tests.

    async fn create(
        &self,
        _module: edgelet_settings::ModuleSpec<Self::Config>,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn get(
        &self,
        _id: &str,
    ) -> Result<(Self::Module, edgelet_core::ModuleRuntimeState), Self::Error> {
        unimplemented!()
    }

    async fn start(&self, _id: &str) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn stop(
        &self,
        _id: &str,
        _wait_before_kill: Option<std::time::Duration>,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn restart(&self, _id: &str) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn remove(&self, _id: &str) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn system_info(&self) -> Result<edgelet_core::SystemInfo, Self::Error> {
        unimplemented!()
    }

    async fn system_resources(&self) -> Result<edgelet_core::SystemResources, Self::Error> {
        unimplemented!()
    }

    async fn list(&self) -> Result<Vec<Self::Module>, Self::Error> {
        unimplemented!()
    }

    async fn list_with_details(
        &self,
    ) -> Result<Vec<(Self::Module, edgelet_core::ModuleRuntimeState)>, Self::Error> {
        unimplemented!()
    }

    async fn logs(&self, _id: &str, _options: &edgelet_core::LogOptions) -> Self::Logs {
        unimplemented!()
    }

    async fn remove_all(&self) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn stop_all(
        &self,
        _wait_before_kill: Option<std::time::Duration>,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn registry(&self) -> &Self::ModuleRegistry {
        unimplemented!()
    }
}
