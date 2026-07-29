#!/usr/bin/env bash
#SBATCH --job-name=qh129-build-smoke
#SBATCH --partition=xhacnormalb
#SBATCH --account=giggleliu
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=56
#SBATCH --mem=196G
#SBATCH --time=00:45:00
#SBATCH --output=/work/share/giggleliu/cfys01/quantum-harness-129/slurm-%j.out
#SBATCH --error=/work/share/giggleliu/cfys01/quantum-harness-129/slurm-%j.err

set -euo pipefail
unset LC_ALL
export LANG=C

: "${QH129_ORCHESTRATION:?submit with --export=ALL,QH129_ORCHESTRATION=/absolute/path}"
source "${QH129_ORCHESTRATION}/common.sh"

readonly RUN_DIR="${QH129_RUN_ROOT}/${SLURM_JOB_ID}/build-smoke"
mkdir -p "${RUN_DIR}"

qh129_finish() {
    local status=$?
    set +e
    printf '%s\n' "${status}" >"${RUN_DIR}/exit-status.txt"
    printf '%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >"${RUN_DIR}/ended-at.txt"
    qh129_record_environment "${RUN_DIR}/environment-final.env"
    exit "${status}"
}
trap qh129_finish EXIT

printf '%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >"${RUN_DIR}/started-at.txt"

source /usr/share/Modules/init/bash
module purge
module load compiler/gcc/11.4.0

qh129_verify_source
qh129_require_toolchain
qh129_record_environment "${RUN_DIR}/environment-build.env"

export CARGO_BUILD_JOBS=16
export RAYON_NUM_THREADS=56

(
    cd "${QH129_SOURCE}"
    /usr/bin/time -v -o "${RUN_DIR}/cargo-build.time" \
        cargo build --release --locked --offline
    /usr/bin/time -v -o "${RUN_DIR}/cargo-test.time" \
        cargo test --locked --all-targets --offline
)

test -x "${QH129_BINARY}"
qh129_record_environment "${RUN_DIR}/environment-binary.env"

"${QH129_BINARY}" verify \
    "${QH129_SOURCE}/fixtures/h2-equilibrium-sto3g/FCIDUMP" \
    "${QH129_SOURCE}/fixtures/h2-equilibrium-sto3g/reference.json" \
    >"${RUN_DIR}/verify-h2-equilibrium.stdout" \
    2>"${RUN_DIR}/verify-h2-equilibrium.stderr"

"${QH129_BINARY}" verify \
    "${QH129_SOURCE}/fixtures/h4-sto3g/FCIDUMP" \
    "${QH129_SOURCE}/fixtures/h4-sto3g/reference.json" \
    >"${RUN_DIR}/verify-h4.stdout" \
    2>"${RUN_DIR}/verify-h4.stderr"

"${QH129_BINARY}" verify \
    "${QH129_SOURCE}/fixtures/h2o-sto3g/FCIDUMP" \
    "${QH129_SOURCE}/fixtures/h2o-sto3g/reference.json" \
    >"${RUN_DIR}/verify-h2o-sto3g.stdout" \
    2>"${RUN_DIR}/verify-h2o-sto3g.stderr"

/usr/bin/time -v -o "${RUN_DIR}/ccpvdz.time" \
    "${QH129_BINARY}" benchmark h2o-cc-pvdz \
    --sources 16 \
    --memory-budget-gib 2 \
    --json-output "${RUN_DIR}/h2o-ccpvdz-ae.json" \
    >"${RUN_DIR}/ccpvdz.stdout" \
    2>"${RUN_DIR}/ccpvdz.stderr"

grep -F '"full_fci_executed": false' "${RUN_DIR}/h2o-ccpvdz-ae.json"

/usr/bin/time -v -o "${RUN_DIR}/davidson.time" \
    "${QH129_BINARY}" davidson "${QH129_FCIDUMP}" \
    --residual-tolerance 1e-7 \
    --max-iterations 60 \
    --max-subspace 20 \
    --parallel-blocks 56 \
    --parallel-memory-budget-gib 2 \
    --strict-parallel-memory \
    >"${RUN_DIR}/davidson.stdout" \
    2>"${RUN_DIR}/davidson.stderr"

grep -F -- '-76.12117' "${RUN_DIR}/davidson.stdout"

(
    cd "${RUN_DIR}"
    sha256sum \
        environment-build.env \
        environment-binary.env \
        h2o-ccpvdz-ae.json \
        davidson.stdout \
        verify-h2-equilibrium.stdout \
        verify-h4.stdout \
        verify-h2o-sto3g.stdout \
        >SHA256SUMS
)
