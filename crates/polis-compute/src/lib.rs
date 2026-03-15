pub struct ComputeModule;

impl ComputeModule {
    pub const fn name() -> &'static str {
        "polis-compute"
    }
}

/// Compute backend selection for acceleration-sensitive kernels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeBackend {
    /// Authoritative deterministic path.
    CpuReference,
    /// Optional accelerated path (must remain parity-checked against CPU).
    GpuAccelerated,
}

/// Runtime compute configuration for Phase 8 selective acceleration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComputeConfig {
    pub backend: ComputeBackend,
    /// Keep deterministic execution contract even when using acceleration.
    pub deterministic_mode: bool,
    /// Maximum batch size eligible for GPU kernels.
    pub gpu_batch_threshold: usize,
    /// Allowed relative error in parts-per-million against CPU reference.
    pub tolerance_ppm: u32,
}

impl Default for ComputeConfig {
    fn default() -> Self {
        Self {
            backend: ComputeBackend::CpuReference,
            deterministic_mode: true,
            gpu_batch_threshold: 4_096,
            tolerance_ppm: 10,
        }
    }
}

/// Core compute facade used by simulation systems.
#[derive(Debug, Clone, Copy)]
pub struct ComputeEngine {
    pub config: ComputeConfig,
}

impl ComputeEngine {
    pub fn new(config: ComputeConfig) -> Self {
        Self { config }
    }

    /// 1D ring diffusion kernel.
    /// This is the first selective-acceleration candidate for dense field math.
    pub fn diffuse_ring_f32(
        &self,
        input: &[f32],
        diffusion_rate: f32,
        output: &mut [f32],
    ) -> ComputeBackend {
        assert_eq!(input.len(), output.len(), "input/output length mismatch");
        assert!((0.0..=1.0).contains(&diffusion_rate), "invalid diffusion_rate");

        match self.config.backend {
            ComputeBackend::CpuReference => {
                diffuse_ring_f32_cpu(input, diffusion_rate, output);
                ComputeBackend::CpuReference
            }
            ComputeBackend::GpuAccelerated => {
                // Phase 8 scaffold: GPU path currently delegates to parity-safe CPU kernel.
                diffuse_ring_f32_cpu(input, diffusion_rate, output);
                ComputeBackend::GpuAccelerated
            }
        }
    }

    /// Deterministic reduction kernel candidate for acceleration.
    pub fn reduce_sum_u64(&self, values: &[u64]) -> (u64, ComputeBackend) {
        let sum = reduce_sum_u64_cpu(values);
        let backend = match self.config.backend {
            ComputeBackend::CpuReference => ComputeBackend::CpuReference,
            // Phase 8 scaffold: same reduction path, backend tag reserved for GPU implementation.
            ComputeBackend::GpuAccelerated => ComputeBackend::GpuAccelerated,
        };
        (sum, backend)
    }

    /// Deterministic floating-point reduction kernel candidate for acceleration.
    pub fn reduce_sum_f32(&self, values: &[f32]) -> (f32, ComputeBackend) {
        let sum = reduce_sum_f32_cpu(values);
        let backend = match self.config.backend {
            ComputeBackend::CpuReference => ComputeBackend::CpuReference,
            // Phase 8 scaffold: backend tag reserved for GPU implementation.
            ComputeBackend::GpuAccelerated => ComputeBackend::GpuAccelerated,
        };
        (sum, backend)
    }
}

/// CPU reference diffusion implementation (authoritative contract).
pub fn diffuse_ring_f32_cpu(input: &[f32], diffusion_rate: f32, output: &mut [f32]) {
    let n = input.len();
    if n == 0 {
        return;
    }
    if n == 1 {
        output[0] = input[0];
        return;
    }

    for i in 0..n {
        let prev = input[(i + n - 1) % n];
        let current = input[i];
        let next = input[(i + 1) % n];
        let neighbor_avg = (prev + next) * 0.5;
        output[i] = current + diffusion_rate * (neighbor_avg - current);
    }
}

/// CPU reference reduction implementation.
pub fn reduce_sum_u64_cpu(values: &[u64]) -> u64 {
    values.iter().fold(0_u64, |acc, v| acc.wrapping_add(*v))
}

/// CPU reference floating-point reduction implementation.
pub fn reduce_sum_f32_cpu(values: &[f32]) -> f32 {
    // Kahan summation for improved numeric stability while preserving deterministic order.
    let mut sum = 0.0_f32;
    let mut c = 0.0_f32;
    for v in values {
        let y = *v - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diffusion_preserves_uniform_field() {
        let input = vec![3.0_f32; 64];
        let mut output = vec![0.0_f32; 64];
        diffuse_ring_f32_cpu(&input, 0.2, &mut output);
        assert!(output.iter().all(|v| (*v - 3.0).abs() < 1e-6));
    }

    #[test]
    fn diffusion_reduces_local_gradient() {
        let input = vec![0.0, 0.0, 10.0, 0.0, 0.0];
        let mut output = vec![0.0_f32; input.len()];
        diffuse_ring_f32_cpu(&input, 0.5, &mut output);
        assert!(output[2] < input[2]);
        assert!(output[1] > input[1]);
        assert!(output[3] > input[3]);
    }

    #[test]
    fn cpu_and_gpu_modes_match_for_scaffold_kernels() {
        let input = (0..128).map(|i| (i % 7) as f32).collect::<Vec<_>>();
        let mut cpu_out = vec![0.0_f32; input.len()];
        let mut gpu_out = vec![0.0_f32; input.len()];

        let cpu_engine = ComputeEngine::new(ComputeConfig::default());
        let mut gpu_cfg = ComputeConfig::default();
        gpu_cfg.backend = ComputeBackend::GpuAccelerated;
        let gpu_engine = ComputeEngine::new(gpu_cfg);

        cpu_engine.diffuse_ring_f32(&input, 0.15, &mut cpu_out);
        gpu_engine.diffuse_ring_f32(&input, 0.15, &mut gpu_out);

        assert_eq!(cpu_out, gpu_out);

        let values = vec![1_u64, 2, 3, 10, 100, 1_000];
        assert_eq!(cpu_engine.reduce_sum_u64(&values).0, 1_116);
        assert_eq!(
            cpu_engine.reduce_sum_u64(&values).0,
            gpu_engine.reduce_sum_u64(&values).0
        );

        let float_values = vec![0.1_f32, 1.5, 2.25, 10.0];
        let cpu_sum = cpu_engine.reduce_sum_f32(&float_values).0;
        let gpu_sum = gpu_engine.reduce_sum_f32(&float_values).0;
        assert!((cpu_sum - gpu_sum).abs() < 1e-6);
    }
}
