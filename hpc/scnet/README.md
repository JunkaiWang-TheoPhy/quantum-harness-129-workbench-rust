# SCNet HPC benchmark

These scripts reproduce the v0.4.0 workbench on the authorized SCNet
allocation and run an 18-case H2O/6-31G frozen-core Davidson robustness matrix.
Each case uses one 56-core node, 56 Rayon workers, and 56 fixed source blocks.
At full array concurrency the experiment uses 1,008 CPU cores.

This is task-parallel ensemble throughput.  The v0.4.0 solver is
shared-memory-only, so the experiment is not a claim that one Davidson solve
uses 1,008 cores.

## Fixed inputs

- source commit: `48f1964a1b3b88090497e1ffce285fde09c98541`
- Rust: `1.89.0`
- account: `giggleliu`
- partition: `xhacnormalb`
- remote root:
  `/work/share/giggleliu/cfys01/quantum-harness-129`

The remote source checkout must be detached at the fixed commit and clean.
Orchestration scripts are staged outside that checkout.

## Stage source and scripts

From an authenticated machine, create the remote root and clone the fixed
source:

```bash
ssh SCNET 'mkdir -p /work/share/giggleliu/cfys01/quantum-harness-129 &&
  if test ! -d /work/share/giggleliu/cfys01/quantum-harness-129/source-v0.4.0/.git; then
    git clone https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust.git \
      /work/share/giggleliu/cfys01/quantum-harness-129/source-v0.4.0
  fi &&
  cd /work/share/giggleliu/cfys01/quantum-harness-129/source-v0.4.0 &&
  git checkout --detach 48f1964a1b3b88090497e1ffce285fde09c98541'

scp -r hpc/scnet SCNET:/work/share/giggleliu/cfys01/quantum-harness-129/orchestration-v1
```

`SCNET` denotes the user's configured SSH command or host alias.  Private-key
paths are intentionally not stored in this repository.

## Submit the gated run

Submit build/smoke first:

```bash
ssh SCNET 'cd /work/share/giggleliu/cfys01/quantum-harness-129/orchestration-v1 &&
  sbatch --parsable build-smoke.sbatch'
```

Require a zero exit status and inspect:

```text
runs/<job-id>/build-smoke/
```

Only after that gate passes, submit production:

```bash
ssh SCNET 'cd /work/share/giggleliu/cfys01/quantum-harness-129/orchestration-v1 &&
  sbatch --parsable davidson-robustness.sbatch'
```

The array header is `0-17%18`; each task requests 56 CPUs.  Slurm can therefore
allocate at most `18 × 56 = 1,008` CPUs to this array.

## Failure and resubmission

Every task writes an isolated directory below:

```text
runs/<array-job-id>/davidson-array/<array-index>/
```

Inspect `exit-status.txt`, `davidson.stderr`, and Slurm terminal state.
Resubmit only failed indices, for example:

```bash
sbatch --array=7,11 davidson-robustness.sbatch
```

Remote results are retained.  Download them without deleting or moving the
remote copy.
