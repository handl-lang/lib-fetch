# lib-fetch

The `fetch` port contract — WinterCG-style request/response surface implemented by `fetch-c` (libcurl) and `fetch-js` (WebIDL bindings).

## Layout

```text
src/                     the .hnl sources
pkg.hnl                  package manifest
scripts/hnl-gate.sh      the gate
ci.sh                    bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
