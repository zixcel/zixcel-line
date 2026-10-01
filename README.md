# zixcel-line

Configuration validation and planning for observing LINE delivery statistics, audience aggregates and webhook-event metadata. Plans exclude personal message bodies and user identifiers.

```bash
cargo run --offline -- doctor
cargo run --offline -- capabilities
cargo run --offline -- validate examples/config.toml
cargo run --offline -- plan examples/config.toml
```

Only `secret://...` references are allowed; authentication values are not stored. The CLI performs no networking, credential resolution, webhook reception or message transmission.

Library use combines `parse_config` and `build_plan` with closed TOML up to 1 MiB and 32 nesting levels. Message bodies and user identifiers are not allowlisted; communication and credential resolution are not implemented. The crate uses `publish = false` during local validation.

## Quality gate

These checks run within this crate without connecting to LINE APIs.

```bash
# WONDERLAND_ROOT is the workspace checkout root.
"$WONDERLAND_ROOT/bin/verify-repositories" --rust --tier standard
```

## Package integration

The package is an independently consumable unit. Callers reference its documented
interface through a versioned dependency and own application-specific composition
and integration.

## Distribution license

Apache-2.0. Copyright 2026 HAT Inc. See [LICENSE](LICENSE) and [NOTICE](NOTICE). Earlier license files and third-party terms remain applicable to their respective portions.
