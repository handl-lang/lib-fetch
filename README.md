# lib-fetch

The `fetch` port contract — WinterCG-style request/response surface implemented by `fetch-c` (libcurl) and `fetch-js` (WebIDL bindings).

Part of the `handl-lang` org split (ADR-0128). The contract is extracted
from the backend packages' documented surfaces — no prior history.

## Layout

```text
src/        the .hnl sources
pkg.hnl     dependencies, when any: `{ deps: { <name>: "branch:<ref>" } }`
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
