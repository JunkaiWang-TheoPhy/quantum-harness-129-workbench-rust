# Rager PR Identity Redesign

## Scope

Update the five open Quantum Harness pull requests authored by
`JunkaiWang-TheoPhy` whose titles or bodies identify the team as
`Rewrite It In Rust!`:

- `QuantumBFS/quantum.harness#214`
- `QuantumBFS/quantum.harness#215`
- `QuantumBFS/quantum.harness#216`
- `QuantumBFS/quantum.harness#217`
- `QuantumBFS/quantum.harness#220`

The closed and superseded PR `#210` is explicitly out of scope.

## Copy

Replace team-name uses of `Rewrite It In Rust!`, capitalization variants, and
the `RIIR` abbreviation with `Rager` in each PR title and body. Preserve
technical content, completion dates, tables, links, evidence, and validation
records. Remove the redundant trailing phrase in PR #220 rather than changing
it into the ungrammatical `and Rager`.

Every body begins with this exact Markdown block quote:

```markdown
> Do not go gentle into that good night,
> Old age should burn and rave at close of day;
> Rage, rage against the dying of the light.
> —— [Dylan Thomas](https://www.poetryfoundation.org/poets/dylan-thomas), 「**Do Not Go Gentle into That Good Night**」
```

The PR-specific banner follows the quotation and precedes the original body.

## Visual System

Generate five original 21:9 cinematic space banners with GPT Image 2. The
images may evoke the scale, solitude, celestial light, and exploratory tension
associated with *Interstellar*, but must not reproduce actors, characters,
logos, typography, costumes, spacecraft designs, or identifiable film shots.
No banner contains text.

| PR | Banner concept |
|---|---|
| #214 | A luminous tensor lattice bending around a black hole |
| #215 | A minimal glowing circuit across a frozen alien world |
| #216 | A crystalline proof lattice inside monumental cosmic architecture |
| #217 | An accretion disk surrounded by determinant-like stellar states and Hamiltonian orbital paths |
| #220 | A solitary original spacecraft performing a gravity slingshot past a ringed planet |

Each concept uses a different composition and dominant palette so that the
banners are related but visibly distinct.

## Hosting

Store the generated images on a dedicated `media/rager-pr-banners` branch in
the user's `JunkaiWang-TheoPhy/quantum.harness` fork. Link them through stable
raw GitHub URLs. This keeps binary media out of all five implementation PR
diffs while retaining control of the assets.

## Verification

After editing, retrieve all five PRs again and verify:

- every title contains `Rager`;
- no title or body contains `Rewrite It In Rust`, capitalization variants, or
  the standalone `RIIR` team abbreviation;
- every body starts with the exact four-line Markdown quote;
- every body contains exactly one unique Rager banner URL;
- the original technical body remains after the inserted header;
- PR #210's title and body remain unchanged.

