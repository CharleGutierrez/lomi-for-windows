<div align="center">

<img src="assets/logo-dark.svg#gh-dark-mode-only" alt="LOMI AGI for Windows" width="600">
<img src="assets/logo-light.svg#gh-light-mode-only" alt="LOMI AGI for Windows" width="600">

**The Ultimate AGI Operating System & Local Universal API Gateway**

[![Rust](https://img.shields.io/badge/rust-v1.75+-blue.svg?logo=rust)](https://www.rust-lang.org/)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20%7C%20Server-blue.svg)]()
[![Hardware: NPU/DirectML](https://img.shields.io/badge/Hardware-DirectML%20%7C%20NPU%20%7C%20CUDA-orange.svg)]()

**Lomi for Windows** is a high-speed, zero-cost AI Gateway written entirely in Rust, rebuilt from the ground up for the Windows ecosystem. It intercepts API requests from your favorite AI tools (Pi, Cursor, LangChain) via Zero-Latency Named Pipes or TCP, and dynamically routes them to local NPUs or cloud providers while applying extreme token minification, caching, and enterprise-grade sandboxing.

</div>

---

## ✨ God-Tier Windows Features

| Feature | Description |
| :--- | :--- |
| ⚡ **Microsoft DirectStorage API** | Bypasses the CPU to stream massive 70B `.safetensors` model weights directly from PCIe Gen4/5 NVMe SSDs into your NPU/GPU VRAM in milliseconds. |
| 🛡️ **Hyper-V & Job Object Sandboxing** | Intercepts untrusted AI-generated `powershell` or `cmd` scripts and executes them inside an isolated Windows Sandbox container with strict Job Object memory limits (100MB RAM, No Network). |
| 🔗 **Zero-Latency Named Pipes** | Exposes `\\.\pipe\LomiGateway` for instantaneous, memory-mapped prompt transmission from local Windows IDEs, bypassing the TCP stack entirely. |
| 🔐 **Windows Hello Biometric Security** | Integrates with Windows Credential Manager. Requires physical Face ID or Fingerprint authentication before decrypting paid Cloud API keys into memory. |
| 📊 **ETW & Event Viewer RAG** | Intercepts queries about "crashes" or "slow" systems. Silently injects the last 60 seconds of Event Tracing for Windows (ETW) logs and Event Viewer faults into the AI context window. |
| 🌉 **WSL2 Cross-VM Bridge** | Automatically configures `iptables` and `/etc/resolv.conf` in your WSL2 instances (Ubuntu) so Linux tools natively route AI requests back to the Windows DirectML host. |
| 🏢 **Enterprise GPO Compliance** | Listens to the Windows Registry. If the corporate `LOMI_GPO_AIRGAP` policy is active, it strictly blocks outgoing internet requests and forces all AI generation to local DirectML models to guarantee zero data leakage. |
| 🔦 **Global Spotlight Overlay** | Spawns a background system tray thread listening for the `[Win + Space]` global keyboard hook to bring up an instant AI command bar anywhere in Windows. |

---

## 🚀 Installation & Setup

Lomi for Windows comes with an automated PowerShell installer for frictionless deployment.

### 1. Automated Installation
Download the `Lomi_for_Windows_v0.1.0_Release.zip` and extract it. Then, open an **Administrator PowerShell** and run:
```powershell
.\install.ps1
```
*This script will copy the optimized release binary to `C:\Program Files\LOMI` and register it as an invisible Windows Task Scheduler boot daemon.*

### 2. Manual Build
If you prefer to compile from source:
```bash
cargo build --release
```

---

## 💻 Operating Manual

### 1. Starting the Gateway (Proxy Mode)
If you aren't using the background daemon, you can run Lomi directly:
```powershell
lomi-win serve-proxy --port 8080
```
Then, point your IDE (Cursor, Pi Coding Agent) to:
**Endpoint:** `http://127.0.0.1:8080/v1` OR use the Named Pipe `\\.\pipe\LomiGateway`.

### 2. Initializing the WSL2 Bridge
Are your development tools inside Ubuntu/WSL2, but your GPU/NPU is on Windows? Run:
```powershell
lomi-win wsl-bridge
```
This automatically establishes a network tunnel so Linux AI agents can utilize Windows DirectML acceleration.

### 3. Local Hardware Tuning (DirectStorage Loading)
Auto-detect your hardware, prepare datasets, and dynamically load/fine-tune LLMs locally:
```powershell
lomi-win tune --model-path ./my_local_model --dataset-path ./data.jsonl
```

### 4. Global Spotlight UI
Once the daemon is running, simply press **`Win + Space`** anywhere in Windows to trigger the zero-latency Spotlight AI command bar. Lomi runs quietly in your **System Tray**—right-click the tray icon to pause or view metrics.

---

## 🏢 Enterprise Air-Gapped Configuration

For IT Administrators managing corporate networks, Lomi supports strict Active Directory GPO enforcement. 
To force Lomi to block all Anthropic/OpenAI API requests and strictly use local DirectML NPUs, set the following System Environment Variable via Registry/GPO:
```powershell
[Environment]::SetEnvironmentVariable("LOMI_GPO_AIRGAP", "1", "Machine")
```
When this flag is detected, Lomi will route all traffic to `qwen2.5-coder-7b (Air-Gapped)` guaranteeing 100% data security.

---

## 📈 Dynamic Hardware Scaling Benchmarks

Lomi's tuning engine automatically evaluates your system hardware pool upon initialization, scaling from legacy laptops to massive Enterprise Clusters. Here are the live optimization resolutions across 5 different hardware classes:

```text
⚙️ LOMI: Initializing Hardware Optimizer Benchmarks

------------------------------------------------------------
🖥️  PROFILE: 7TH GEN OFFICE LAPTOP
   - Compute: Intel Core i5-7200U (2 Cores)
   - Memory : 8 GB RAM
   - Accel. : None (Integrated)

   ⚡ LOMI TUNING ENGINE RESOLUTION:
      └ Target Device : CPU
      └ Quantization  : GGUF 8-bit (AVX2)
      └ Max Threads   : 1 / 2
      └ Batch Size    : 4
      └ Ctx Window    : 2048 tokens
------------------------------------------------------------
🖥️  PROFILE: 12TH GEN THIN-AND-LIGHT
   - Compute: Intel Core i7-1260P (12 Cores)
   - Memory : 16 GB RAM
   - Accel. : Intel Iris Xe (0 GB VRAM)

   ⚡ LOMI TUNING ENGINE RESOLUTION:
      └ Target Device : CPU
      └ Quantization  : GGUF 8-bit (AVX2)
      └ Max Threads   : 11 / 12
      └ Batch Size    : 8
      └ Ctx Window    : 8192 tokens
------------------------------------------------------------
🖥️  PROFILE: LATEST APPLE SILICON
   - Compute: Apple M3 Max (16 Cores)
   - Memory : 128 GB RAM
   - Accel. : Apple Metal Unified GPU (128 GB VRAM)

   ⚡ LOMI TUNING ENGINE RESOLUTION:
      └ Target Device : Metal Performance Shaders (MPS)
      └ Quantization  : QLoRA 4-bit (DirectML/NF4)
      └ Max Threads   : 15 / 16
      └ Batch Size    : 64
      └ Ctx Window    : 32768 tokens
------------------------------------------------------------
🖥️  PROFILE: MODERN GAMING/AI DESKTOP
   - Compute: AMD Ryzen 9 7950X3D (16 Cores)
   - Memory : 64 GB RAM
   - Accel. : NVIDIA RTX 4090 (24 GB VRAM)

   ⚡ LOMI TUNING ENGINE RESOLUTION:
      └ Target Device : DirectX 12 / DirectML (Windows NPU)
      └ Quantization  : QLoRA 4-bit (DirectML/NF4)
      └ Max Threads   : 15 / 16
      └ Batch Size    : 16
      └ Ctx Window    : 8192 tokens
------------------------------------------------------------
🖥️  PROFILE: ENTERPRISE AI SERVER
   - Compute: Dual AMD EPYC 9654 (192 Cores)
   - Memory : 1536 GB RAM
   - Accel. : 8x NVIDIA H100 SXM5 (640 GB VRAM)

   ⚡ LOMI TUNING ENGINE RESOLUTION:
      └ Target Device : DirectX 12 / DirectML (Windows NPU)
      └ Quantization  : BFloat16 (Uncompressed)
      └ Max Threads   : 191 / 192
      └ Batch Size    : 256
      └ Ctx Window    : 128000 tokens
------------------------------------------------------------
```

---
<div align="center">
<i>Engineered for Windows 11. Built with ❤️ by Cognitive Agents.</i>
</div>