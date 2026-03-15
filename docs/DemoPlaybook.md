# POLIS Demo Playbook

This is the fastest repeatable path to a testable demo run.

## 1. Headless demo run (recommended)

```powershell
cargo run -p polis-app -- --demo --parallel
```

Default `--demo` behavior:

- scenario: `scenarios/demo_v1.ron`
- ticks: `3000` (unless `--ticks` is provided)
- export dir: `target/exports/demo_v1` (unless `--export-dir` is provided)

Expected outputs:

- `target/exports/demo_v1/manifest.json`
- `target/exports/demo_v1/events.jsonl`
- `target/exports/demo_v1/metrics.jsonl`
- `target/exports/demo_v1/bundle-index.json`
- `target/exports/demo_v1/checkpoint.json` (optional; emitted when checkpoint serialization succeeds)

## 2. Quick demo checks

```powershell
Get-Content target/exports/demo_v1/manifest.json
Get-Content target/exports/demo_v1/events.jsonl -TotalCount 5
Get-Content target/exports/demo_v1/metrics.jsonl -Tail 5
```

You should see:

- non-zero `event_count`
- `metric_count` equal to ticks
- changing macro metrics over time

## 3. Windowed demo

```powershell
cargo run -p polis-app --features windowed -- --demo --windowed
```

## 4. Manual scenario override

```powershell
cargo run -p polis-app -- --scenario scenarios/default.ron --ticks 1500 --parallel --export-dir target/exports/custom
```
