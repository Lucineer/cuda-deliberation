# cuda-deliberation

**Consider/Resolve/Forfeit - the decision-making protocol.**

> An agent that never deliberates is a reflex machine.
> An agent that always deliberates is paralyzed.

## The Protocol

Every decision in the fleet goes through three stages:

1. **Consider** - Generate and evaluate options
2. **Resolve** - Select the best option above confidence threshold
3. **Forfeit** - Abandon decisions that can't reach consensus

### Key Components

- **`DeliberationEngine`** - Core engine with proposal lifecycle
- **`Proposal`** - An option under consideration with confidence
- **Consensus ratios** - Track agreement/disagreement across agents
- **Auto-forfeit** - Automatically abandon deadlocked proposals

## How Confidence Gates Decisions

Proposals below 0.5 confidence are auto-forfeited. Consensus requires 0.85 agreement. This prevents the fleet from acting on uncertain information.

## Ecosystem Integration

- `cuda-confidence` - Proposal confidence tracking
- `cuda-filtration` - Budget-based filtering of deliberation scope
- `cuda-a2a` - Multi-agent deliberation via A2A messages
- `cuda-resolve-agent` - Deliberative agent with expertise boost
- `cuda-goal` - Goals drive deliberation priorities
- `cuda-emotion` - Emotional state modulates deliberation speed

## See Also

- [cuda-resolve-agent](https://github.com/Lucineer/cuda-resolve-agent) - Deliberative agent
- [cuda-convergence](https://github.com/Lucineer/cuda-convergence) - Convergence detection
- [cuda-filtration](https://github.com/Lucineer/cuda-filtration) - Intelligence filtration

## License

MIT OR Apache-2.0