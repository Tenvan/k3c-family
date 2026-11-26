//! Gemeinsamer Kern eines Servers und die `ServerHandler`-Implementierung je MCP-Session. Von Hand statt
//! `#[tool_handler]`, damit jeder `tools/call` durch `observe.rs` läuft.

use std::collections::HashMap;
use std::sync::Arc;

use rmcp::handler::server::prompt::PromptContext;
use rmcp::handler::server::router::prompt::PromptRouter;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, GetPromptRequestParams, GetPromptResponse, Implementation,
    ListPromptsResult, ListToolsResult, PaginatedRequestParams, ServerCapabilities, ServerConfig, Tool,
};
use rmcp::service::{NotificationContext, RequestContext};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::{ErrorData, RoleServer, ServerHandler};

use super::param_hint::Params;
use super::server::Options;
use super::{Resolver, Stats, observe, with_checkout};
use crate::usage::Clock;

/// Alles, was Neustarts überlebt: Optionen, App, Router, Zähler.
pub(super) struct Core<A: Clone + Send + Sync + 'static> {
    pub(super) opts: Options,
    pub(super) app: A,
    pub(super) tools: Arc<ToolRouter<A>>,
    prompts: Option<PromptRouter<A>>,
    /// Katalog für `tools/list`, nach Namen sortiert wie im Go-SDK.
    listed: Vec<Tool>,
    /// Gültige Parameter je Tool.
    pub(super) params: HashMap<String, Params>,
    pub(super) stats: Stats,
    pub(super) resolver: Resolver,
}

impl<A: Clone + Send + Sync + 'static> Core<A> {
    pub(super) fn new(opts: Options, app: A, tools: ToolRouter<A>, prompts: Option<PromptRouter<A>>) -> Core<A> {
        let mut listed = tools.list_all();
        let mut params = HashMap::new();
        for tool in &mut listed {
            let mut known = Params::of(&tool.input_schema);
            if opts.takes_checkout.as_ref().is_some_and(|takes| takes(&tool.name)) {
                let mut schema = (*tool.input_schema).clone();
                with_checkout(&mut schema, &mut known.names);
                tool.input_schema = Arc::new(schema);
            }
            params.insert(tool.name.to_string(), known);
        }
        let clock: Clock = Arc::new(chrono::Local::now);
        let stats = Stats::new(clock, opts.on_start.clone(), opts.on_call.clone(), opts.target.clone());
        register(&stats, &opts.catalog, &listed);
        let resolver = Resolver::new(opts.name.clone(), opts.root.clone());
        Core {
            opts,
            app,
            tools: Arc::new(tools),
            prompts,
            listed,
            params,
            stats,
            resolver,
        }
    }
}

/// Nimmt jedes Tool in die Zähler auf, damit es auch ohne Aufruf dort steht: zuerst in Katalog-Reihenfolge, den Rest
/// wie `listed` (alphabetisch).
fn register(stats: &Stats, catalog: &[String], listed: &[Tool]) {
    let mut open: Vec<&Tool> = listed.iter().collect();
    let add = |tool: &Tool| stats.register(&tool.name, tool.description.as_deref().unwrap_or_default());
    for name in catalog {
        if let Some(at) = open.iter().position(|t| t.name == name.as_str()) {
            add(open.remove(at));
        }
    }
    open.into_iter().for_each(add);
}

/// Dienst einer MCP-Session; die Fabrik von `StreamableHttpService` baut je Session einen.
pub(super) struct KitService<A: Clone + Send + Sync + 'static> {
    core: Arc<Core<A>>,
    sessions: Arc<LocalSessionManager>,
}

impl<A: Clone + Send + Sync + 'static> KitService<A> {
    pub(super) fn new(core: Arc<Core<A>>, sessions: Arc<LocalSessionManager>) -> KitService<A> {
        KitService { core, sessions }
    }
}

impl<A: Clone + Send + Sync + 'static> ServerHandler for KitService<A> {
    fn get_info(&self) -> ServerConfig {
        let mut capabilities = ServerCapabilities::builder().enable_tools().build();
        if self.core.prompts.is_some() {
            capabilities.prompts = Some(Default::default());
        }
        let opts = &self.core.opts;
        ServerConfig::new(capabilities)
            .with_server_info(Implementation::new(opts.name.clone(), opts.version.clone()))
            .with_instructions(opts.instructions.clone())
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        self.core.listed.iter().find(|t| t.name == name).cloned()
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items(self.core.listed.clone()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        observe::call_tool(&self.core, request, context).await
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        let prompts = self.core.prompts.as_ref().map(PromptRouter::list_all);
        Ok(ListPromptsResult::with_all_items(prompts.unwrap_or_default()))
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResponse, ErrorData> {
        let Some(prompts) = &self.core.prompts else {
            let message = format!("unknown prompt {:?}", request.name);
            return Err(ErrorData::invalid_params(message, None));
        };
        let prompt = PromptContext::new(&self.core.app, request.name, request.arguments, context);
        prompts.get_prompt(prompt).await
    }

    async fn on_initialized(&self, _context: NotificationContext<RoleServer>) {
        let clients = self.sessions.sessions.read().await.len();
        self.core.stats.observe_clients(clients);
    }
}
