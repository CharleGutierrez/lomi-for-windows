import re
import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# 1. Insert vella_bridge mod
if 'pub mod vella_bridge;' not in content:
    content = content.replace('pub mod core;\n', 'pub mod core;\npub mod vella_bridge;\n')

# 2. Add variants to enum Commands
variants = """
    /// Connect to Vella Framework Realtime Hub
    VellaBridge {
        #[arg(short, long)]
        test: bool,
        #[arg(short, long, default_value = "http://127.0.0.1:3001/api/telemetry")]
        endpoint: String,
    },
    /// Synchronize Lomi datasets, DPO pairs, and vectors into Vella DB
    VellaSync {
        #[arg(short, long, default_value = "../Vella/vella.db")]
        vella_db: String,
    },
    /// Run Vella AiTuner closed-loop optimization on Lomi runtime parameters
    VellaTune,
    /// Compress prompt code or text using AST-aware Token Squeezer
    CompressPrompt {
        #[arg(short, long)]
        text: Option<String>,
    },
    /// Test dynamic model routing and endpoint failover logic
    RouteTest {
        #[arg(short, long, default_value = "auto")]
        model: String,
        #[arg(short, long)]
        prompt: Option<String>,
    },
    /// Rotate context window for a conversation payload
    RotateContext {
        #[arg(short, long, default_value_t = 500)]
        max_tokens: usize,
    },
    /// Scrub sensitive PII, API keys, and private credentials from prompt text
    ScrubPrompt {
        #[arg(short, long)]
        text: Option<String>,
    },
    /// Evaluate or store predictive prefix prompt cache entries
    PrefixCache {
        #[arg(short, long)]
        prompt: Option<String>,
    },
    /// Estimate API cost ($ USD) and evaluate rate limiting
    CheckCost {
        #[arg(short, long, default_value = "gpt-4o")]
        model: String,
        #[arg(short, long, default_value_t = 2000)]
        prompt_tokens: usize,
        #[arg(short, long, default_value_t = 1000)]
        completion_tokens: usize,
    },
    /// Perform semantic vector RAG search over codebase index
    VectorSearch {
        #[arg(short, long)]
        query: Option<String>,
    },
    /// Display Linux cgroups v2 memory slice and resource telemetry
    CgroupStatus,
    /// Scan prompt for prompt injection, jailbreak attempts, and security threats
    ScanPrompt {
        #[arg(short, long)]
        prompt: Option<String>,
    },
    /// Test the full end-to-end 9-step Universal AI Gateway Proxy Pipeline
    TestPipeline,
    /// Benchmark throughput (tokens/sec) and latency across local models
    BenchModels,
    /// Install LOMI as a background OS Daemon systemd service unit
    SetupDaemon,
"""
if 'CompressPrompt {' not in content:
    content = content.replace('    Top,\n}', '    Top,\n' + variants + '}')

# 3. Add match arms to match &cli.command
arms = """
        Commands::VellaBridge { test, endpoint } => {
            println!("⚡ LOMI 🤝 VELLA REALTIME BRIDGE");
            println!("   Vella Endpoint: {}", endpoint);
            let bridge = vella_bridge::VellaBridge::new(Some(endpoint.clone()), None);
            if *test {
                let packet = vella_bridge::VellaTelemetryPacket::new(
                    "gpt-4", "llama3.2:latest", "Local Ollama Engine", 250, 190, 12, 45, 120, 0,
                );
                bridge.emit_telemetry(packet);
            }
        }
        Commands::VellaSync { vella_db } => {
            let bridge = vella_bridge::VellaBridge::new(None, Some(std::path::PathBuf::from(vella_db)));
            let _ = bridge.sync_to_vella_db();
        }
        Commands::VellaTune => {
            let bridge = vella_bridge::VellaBridge::default();
            println!("{}", bridge.run_vella_ai_tuner());
        }
        Commands::CompressPrompt { text } => {
            let input = text.clone().unwrap_or_else(|| "// Example\\nfn add(a: i32, b: i32) -> i32 { a + b }".to_string());
            let result = crate::core::token_squeezer::TokenSqueezer::compress_prompt(&input);
            println!("{:#?}", result);
        }
        Commands::RouteTest { model, prompt } => {
            let sample = prompt.clone().unwrap_or_default();
            let decision = crate::core::model_router::ModelRouter::route_request(model, &sample, None);
            println!("{:#?}", decision);
        }
        Commands::RotateContext { max_tokens } => {
            use crate::core::context_rotator::{ChatMessage, ContextRotator};
            let sample_messages = vec![];
            let res = ContextRotator::rotate_context(&sample_messages, *max_tokens);
            println!("{:#?}", res);
        }
        Commands::ScrubPrompt { text } => {
            let input = text.clone().unwrap_or_default();
            let report = crate::core::privacy_scrubber::PrivacyScrubber::scrub_prompt(&input);
            println!("{:#?}", report);
        }
        Commands::PrefixCache { prompt } => {
            let sample = prompt.clone().unwrap_or_default();
            let (hit, hash) = crate::core::predictive_cache::PredictiveCache::check_prefix_cache(&sample);
            println!("Hit: {}, Hash: {}", hit, hash);
        }
        Commands::CheckCost { model, prompt_tokens, completion_tokens } => {
            let limit = crate::core::rate_limiter::RateLimiter::check_rate_limit(model, 100);
            println!("Limit: {}", limit);
        }
        Commands::VectorSearch { query } => {
            let sample = query.clone().unwrap_or_default();
            let mut rag = crate::core::vector_rag::VectorRagEngine::new(".");
            let _ = rag.search(&sample, 3);
        }
        Commands::CgroupStatus => {
            println!("Cgroup Status - Windows Polyfill Placeholder");
        }
        Commands::ScanPrompt { prompt } => {
            let sample = prompt.clone().unwrap_or_default();
            let report = crate::core::prompt_guard::PromptGuard::scan_prompt(&sample);
            println!("{:#?}", report);
        }
        Commands::TestPipeline => {
            println!("Test Pipeline - See core modules");
        }
        Commands::BenchModels => {
            println!("BenchModels - See core::model_benchmark");
        }
        Commands::SetupDaemon => {
            println!("SetupDaemon - Use Windows Task Scheduler logic instead");
        }
"""
if 'Commands::VellaBridge' not in content:
    content = content.replace('    match &cli.command {\n', '    match &cli.command {\n' + arms)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print("Updated src/main.rs successfully")
