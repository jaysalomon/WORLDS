use std::time::Instant;

use polis_compute::{ComputeBackend, ComputeConfig, ComputeEngine};

fn main() {
    let sizes = [4_096_usize, 16_384, 65_536];
    let iters = 200_u32;

    let mut cpu_cfg = ComputeConfig::default();
    cpu_cfg.backend = ComputeBackend::CpuReference;
    let cpu = ComputeEngine::new(cpu_cfg);

    let mut gpu_cfg = ComputeConfig::default();
    gpu_cfg.backend = ComputeBackend::GpuAccelerated;
    let gpu = ComputeEngine::new(gpu_cfg);

    println!("POLIS compute perf probe");
    println!("iterations_per_case={iters}");
    println!();

    for size in sizes {
        let input_f32: Vec<f32> = (0..size).map(|i| ((i % 97) as f32) * 0.01).collect();
        let input_u64: Vec<u64> = (0..size).map(|i| (i % 1_000) as u64).collect();

        let mut out_cpu = vec![0.0_f32; size];
        let mut out_gpu = vec![0.0_f32; size];

        let start_cpu_diff = Instant::now();
        for _ in 0..iters {
            cpu.diffuse_ring_f32(&input_f32, 0.15, &mut out_cpu);
        }
        let cpu_diff_ms = start_cpu_diff.elapsed().as_secs_f64() * 1_000.0;

        let start_gpu_diff = Instant::now();
        for _ in 0..iters {
            gpu.diffuse_ring_f32(&input_f32, 0.15, &mut out_gpu);
        }
        let gpu_diff_ms = start_gpu_diff.elapsed().as_secs_f64() * 1_000.0;

        let start_cpu_u64 = Instant::now();
        let mut cpu_u64_acc = 0_u64;
        for _ in 0..iters {
            cpu_u64_acc ^= cpu.reduce_sum_u64(&input_u64).0;
        }
        let cpu_u64_ms = start_cpu_u64.elapsed().as_secs_f64() * 1_000.0;

        let start_gpu_u64 = Instant::now();
        let mut gpu_u64_acc = 0_u64;
        for _ in 0..iters {
            gpu_u64_acc ^= gpu.reduce_sum_u64(&input_u64).0;
        }
        let gpu_u64_ms = start_gpu_u64.elapsed().as_secs_f64() * 1_000.0;

        let start_cpu_f32 = Instant::now();
        let mut cpu_f32_acc = 0.0_f32;
        for _ in 0..iters {
            cpu_f32_acc += cpu.reduce_sum_f32(&input_f32).0;
        }
        let cpu_f32_ms = start_cpu_f32.elapsed().as_secs_f64() * 1_000.0;

        let start_gpu_f32 = Instant::now();
        let mut gpu_f32_acc = 0.0_f32;
        for _ in 0..iters {
            gpu_f32_acc += gpu.reduce_sum_f32(&input_f32).0;
        }
        let gpu_f32_ms = start_gpu_f32.elapsed().as_secs_f64() * 1_000.0;

        let parity_diff = out_cpu == out_gpu;
        let parity_u64 = cpu_u64_acc == gpu_u64_acc;
        let parity_f32 = (cpu_f32_acc - gpu_f32_acc).abs() < 1e-3;

        println!("size={size}");
        println!(
            "  diffusion_ms: cpu={:.3} gpu_tag={:.3} parity={}",
            cpu_diff_ms, gpu_diff_ms, parity_diff
        );
        println!(
            "  reduce_u64_ms: cpu={:.3} gpu_tag={:.3} parity={}",
            cpu_u64_ms, gpu_u64_ms, parity_u64
        );
        println!(
            "  reduce_f32_ms: cpu={:.3} gpu_tag={:.3} parity={}",
            cpu_f32_ms, gpu_f32_ms, parity_f32
        );
    }
}

