use std::fs;
use std::io::Write;
use std::net::TcpListener;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread;

fn get_bin_path() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // remove test binary name
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("lomi-win");
    path.to_str().unwrap().to_string()
}

#[test]
fn test_cmd_test_hardware() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("test-hardware")
        .output()
        .expect("Failed to execute test-hardware");
    
    assert!(output.status.success(), "test-hardware failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- test-hardware output ---\n{}", stdout);
    assert!(stdout.contains("Initializing Hardware Optimizer Benchmarks"));
    assert!(stdout.contains("PROFILE: 7TH GEN OFFICE LAPTOP"));
}

#[test]
fn test_cmd_optimize_pi() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("optimize-pi")
        .arg("--project-path")
        .arg(".")
        .output()
        .expect("Failed to execute optimize-pi");
    
    assert!(output.status.success(), "optimize-pi failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- optimize-pi output ---\n{}", stdout);
    assert!(stdout.contains("Initializing Pi Coding Agent Optimizer"));
    assert!(stdout.contains("PROJECT MEMORY ANALYSIS"));
}

#[test]
fn test_cmd_index() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("index")
        .arg("--path")
        .arg("src")
        .output()
        .expect("Failed to execute index");
    
    assert!(output.status.success(), "index failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- index output ---\n{}", stdout);
    assert!(stdout.contains("Initializing Infinite Memory"));
    assert!(stdout.contains("memorized in"));
}

#[test]
fn test_cmd_web_agent() {
    // Start dummy HTTP server on dynamic port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind server");
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        for _ in 0..10 {
            if let Ok((mut stream, _)) = listener.accept() {
                let body = "<html><body><h1>Lomi Test Page</h1></body></html>";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(), body
                );
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.flush();
                break;
            }
        }
    });

    thread::sleep(Duration::from_millis(200));

    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("web-agent")
        .arg("--url")
        .arg(format!("http://127.0.0.1:{}", port))
        .output()
        .expect("Failed to execute web-agent");
    
    assert!(output.status.success(), "web-agent failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- web-agent output ---\n{}", stdout);
    assert!(stdout.contains("AUTONOMOUS WEB AGENT"));
    assert!(stdout.contains("Web scraping completed"));
}

#[test]
fn test_cmd_swarm_host_and_join() {
    let bin = get_bin_path();

    // Test Host standalone
    let mut host_child = Command::new(&bin)
        .arg("swarm")
        .arg("--mode")
        .arg("host")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn swarm host");

    thread::sleep(Duration::from_millis(500));

    // Test Join standalone while host is running
    let join_output = Command::new(&bin)
        .arg("swarm")
        .arg("--mode")
        .arg("join")
        .output()
        .expect("Failed to execute swarm join");

    let _ = host_child.kill();

    let join_stderr = String::from_utf8_lossy(&join_output.stderr);
    let join_stdout = String::from_utf8_lossy(&join_output.stdout);
    println!("--- swarm join stdout ---\n{}", join_stdout);
    println!("--- swarm join stderr ---\n{}", join_stderr);

    assert!(join_output.status.success(), "Swarm join failed when host was running: {}", join_stderr);
}

#[test]
fn test_cmd_wsl_bridge() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("wsl-bridge")
        .output()
        .expect("Failed to execute wsl-bridge");
    
    assert!(output.status.success(), "wsl-bridge failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- wsl-bridge output ---\n{}", stdout);
    assert!(stdout.contains("WSL2 BRIDGE"));
}

#[test]
fn test_cmd_install_daemon() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("install-daemon")
        .output()
        .expect("Failed to execute install-daemon");
    
    assert!(output.status.success(), "install-daemon failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- install-daemon output ---\n{}", stdout);
    assert!(stdout.contains("LOMI OS DAEMONIZATION"));
    assert!(fs::metadata("lomi_service.xml").is_ok());
}

#[test]
fn test_cmd_iot_bridge() {
    let bin = get_bin_path();
    for cmd_name in &["iotbridge", "io-tbridge", "io-t-bridge"] {
        let output = Command::new(&bin)
            .arg(cmd_name)
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute {}", cmd_name));

        assert!(output.status.success(), "Subcommand {} failed: {}", cmd_name, String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("--- {} output ---\n{}", cmd_name, stdout);
        assert!(stdout.contains("Local IoT mDNS Bridge"));
    }
}

#[test]
fn test_cmd_gpu_kernel() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("gpu-kernel")
        .output()
        .expect("Failed to execute gpu-kernel");
    
    assert!(output.status.success(), "gpu-kernel failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- gpu-kernel output ---\n{}", stdout);
    assert!(stdout.contains("Direct GPU Kernel Programming"));
}

#[test]
fn test_cmd_tune() {
    let temp_dir = std::env::temp_dir().join("lomi_test_tune");
    let _ = fs::create_dir_all(&temp_dir);

    let config_path = temp_dir.join("config.json");
    fs::write(&config_path, r#"{"model_type": "llama", "num_hidden_layers": 12}"#).unwrap();

    let dataset_path = temp_dir.join("dataset.jsonl");
    fs::write(&dataset_path, "{\"instruction\": \"test\", \"output\": \"test output\"}\n").unwrap();

    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("tune")
        .arg("--model-path")
        .arg(temp_dir.to_str().unwrap())
        .arg("--dataset-path")
        .arg(dataset_path.to_str().unwrap())
        .output()
        .expect("Failed to execute tune");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- tune output ---\n{}", stdout);
    assert!(output.status.success(), "tune subcommand failed");
    assert!(stdout.contains("Fine-tuning completed") || stdout.contains("Epoch"));

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_cmd_serve_proxy() {
    let bin = get_bin_path();
    let mut child = Command::new(&bin)
        .arg("serve-proxy")
        .arg("--port")
        .arg("8991")
        .arg("--dashboard-port")
        .arg("3099")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn serve-proxy");

    thread::sleep(Duration::from_millis(1000));

    // Test proxy chat completion endpoint on port 8991
    let client = reqwest::blocking::Client::new();
    let body_str = r#"{"model": "gpt-4o", "messages": [{"role": "user", "content": "Hello Lomi"}]}"#;
    let proxy_res = client.post("http://127.0.0.1:8991/v1/chat/completions")
        .header("Content-Type", "application/json")
        .body(body_str)
        .send();
    assert!(proxy_res.is_ok(), "Failed to connect to proxy endpoint on port 8991");
    if let Ok(res) = proxy_res {
        println!("Proxy chat endpoint status: {}", res.status());
        let text = res.text().unwrap_or_default();
        println!("Proxy chat response: {}", text);
    }

    // Test web dashboard metrics endpoint on custom dashboard port 3099
    let metrics_res = client.get("http://127.0.0.1:3099/api/metrics").send();
    assert!(metrics_res.is_ok(), "Failed to connect to web dashboard metrics endpoint on port 3099");
    if let Ok(res) = metrics_res {
        println!("Web dashboard metrics status: {}", res.status());
        let text = res.text().unwrap_or_default();
        println!("Web dashboard metrics response: {}", text);
        assert!(text.contains("total_tokens_saved"));
    }

    let _ = child.kill();
}

#[test]
fn test_cmd_genesis() {
    let bin = get_bin_path();
    let output = Command::new(&bin)
        .arg("genesis")
        .output()
        .expect("Failed to execute genesis");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- genesis output ---\n{}", stdout);
    assert!(output.status.success(), "genesis subcommand failed");
    assert!(stdout.contains("LOMI GENESIS") || stdout.contains("Self-Improvement"));

    // Cleanup any appended genesis protocol comment lines at the end of file
    if let Ok(content) = fs::read_to_string("src/main.rs") {
        let mut lines: Vec<&str> = content.lines().collect();
        while let Some(last) = lines.last() {
            if last.trim().starts_with("// [LOMI GENESIS PROTOCOL]") || last.trim().is_empty() {
                lines.pop();
            } else {
                break;
            }
        }
        let mut new_content = lines.join("\n");
        new_content.push('\n');
        let _ = fs::write("src/main.rs", new_content);
    }
}

#[test]
fn test_cmd_auto_heal() {
    let bin = get_bin_path();
    let mut child = Command::new(&bin)
        .arg("auto-heal")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn auto-heal");

    thread::sleep(Duration::from_millis(500));

    let _ = child.kill();
    println!("--- auto-heal ---\nSpawned and terminated successfully.");
}

#[test]
fn test_cmd_spotlight() {
    let bin = get_bin_path();
    let mut child = Command::new(&bin)
        .arg("spotlight")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn spotlight");

    thread::sleep(Duration::from_millis(500));

    let _ = child.kill();
    println!("--- spotlight ---\nSpawned and terminated successfully.");
}

#[test]
fn test_cmd_top() {
    let bin = get_bin_path();
    let mut child = Command::new(&bin)
        .arg("top")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn top");

    thread::sleep(Duration::from_millis(500));

    let _ = child.kill();
    println!("--- top ---\nSpawned and terminated successfully.");
}

#[test]
fn test_cmd_gui() {
    let bin = get_bin_path();
    let mut child = Command::new(&bin)
        .arg("gui")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn gui");

    thread::sleep(Duration::from_millis(500));

    let _ = child.kill();
    println!("--- gui ---\nSpawned and terminated successfully.");
}
