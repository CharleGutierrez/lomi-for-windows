import re

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Fix PrefixCache
bad_prefix = '''        Commands::PrefixCache { prompt } => {
            let sample = prompt.clone().unwrap_or_default();
            let (hit, hash) = crate::core::predictive_cache::PredictiveCache::check_prefix_cache(&sample);
            println!("Hit: {}, Hash: {}", hit, hash);
        }'''
good_prefix = '''        Commands::PrefixCache { prompt } => {
            let sample = prompt.clone().unwrap_or_else(|| "You are LOMI AI Operating System Assistant v1.0.".to_string());
            let eval = crate::core::predictive_cache::PredictiveCache::evaluate_prefix(&sample);
            println!("🔮 LOMI PREDICTIVE PREFIX PROMPT CACHE:\\n============================================================");
            println!("Prefix Hash : {:x}", eval.prefix_hash);
            println!("Cache Status: {}", if eval.is_hit { "HIT ⚡" } else { "MISS (Storing new prefix)" });
        }'''
content = content.replace(bad_prefix, good_prefix)

# Fix CheckCost
bad_cost = '''        Commands::CheckCost { model, prompt_tokens, completion_tokens } => {
            let limit = crate::core::rate_limiter::RateLimiter::check_rate_limit(model, 100);
            println!("Limit: {}", limit);
        }'''
good_cost = '''        Commands::CheckCost { model, prompt_tokens, completion_tokens } => {
            let cost = crate::core::rate_limiter::RateLimiter::evaluate("cli_user", model, *prompt_tokens, *completion_tokens, 60);
            println!("💰 LOMI TOKEN-BUCKET RATE LIMITER & COST METER:\\n============================================================");
            println!("Estimated Cost  : ${:.6} USD {}", cost.estimated_cost_usd, if cost.is_local_free_compute { "(FREE Local Inference)" } else { "" });
            println!("RPM Status      : {}/{} RPM ({})", cost.current_rpm, cost.max_rpm, if cost.rate_limit_allowed { "ALLOWED ✅" } else { "BLOCKED 🛑" });
        }'''
content = content.replace(bad_cost, good_cost)

# Fix VectorSearch
bad_vector = '''        Commands::VectorSearch { query } => {
            let sample = query.clone().unwrap_or_default();
            let mut rag = crate::core::vector_rag::VectorRagEngine::new(".");
            let _ = rag.search(&sample, 3);
        }'''
good_vector = '''        Commands::VectorSearch { query } => {
            let sample = query.clone().unwrap_or_else(|| "memory tuner optimization".to_string());
            let results = crate::core::vector_rag::VectorRagEngine::search_codebase(&sample, 5);
            println!("🔍 LOMI INFINITE VECTOR RAG SEARCH:\\n============================================================");
            for r in results.iter() {
                println!("Score: {:.3} | Path: {}", r.similarity_score, r.doc_path);
            }
        }'''
content = content.replace(bad_vector, good_vector)

# Remove unused context_rotator chat message import
content = content.replace('use crate::core::context_rotator::{ChatMessage, ContextRotator};', 'use crate::core::context_rotator::ContextRotator;')

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print("Updated API calls")
