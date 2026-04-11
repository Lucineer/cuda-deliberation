# cuda-deliberation

Consider/Resolve/Forfeit deliberation protocol — multi-agent consensus engine with Bayesian confidence

Part of the Cocapn cognitive layer — how agents think, decide, and learn.

## What It Does

### Key Types

- `Proposal` — core data structure
- `DeliberationEngine` — core data structure
- `ProposalSummary` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-deliberation.git
cd cuda-deliberation

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_deliberation::*;

// See src/lib.rs for full API
// 8 unit tests included
```

### Available Implementations

- `Proposal` — see source for methods
- `DeliberationEngine` — see source for methods

## Testing

```bash
cargo test
```

8 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: cognition
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-confidence-cascade](https://github.com/Lucineer/cuda-confidence-cascade)
- [cuda-reflex](https://github.com/Lucineer/cuda-reflex)
- [cuda-goal](https://github.com/Lucineer/cuda-goal)
- [cuda-fusion](https://github.com/Lucineer/cuda-fusion)
- [cuda-attention](https://github.com/Lucineer/cuda-attention)
- [cuda-emotion](https://github.com/Lucineer/cuda-emotion)
- [cuda-narrative](https://github.com/Lucineer/cuda-narrative)
- [cuda-learning](https://github.com/Lucineer/cuda-learning)
- [cuda-skill](https://github.com/Lucineer/cuda-skill)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
