# lib-fetch

`fetch` — the fetch **port contract**: a WinterCG-style
request/response surface. Implemented by `fetch-c` (libcurl) and
`fetch-js` (WebIDL bindings).

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the contract — request/response surface |
| `pkg.hnl` | manifest — `modules: ["fetch"]` |
| `scripts/hnl-gate.sh` | `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Contract-only repo: declare the `fetch` surface here; backends are
  sibling packages (`lib-fetch-c`, `lib-fetch-js`) depending on
  `fetch` (`branch:main`).

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
