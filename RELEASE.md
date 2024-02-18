# How to release

## Update version

- Edit: `Cargo.toml`
- Run: `cargo update [-w|--workspace]`

## Add tag

```bash
git tag -a v{major}.{minor}.{patch} -m 'foo'
git push origin v{major}.{minor}.{patch}
```

## Publish

[GitHub Actions](./.github/workflows/release.yml) run `cargo publish` automatically.  

- Trigger: [Releases](https://github.com/guricerin/bf-derivatives-tools/releases) -> `Draft a new release` -> `Publish release`  
