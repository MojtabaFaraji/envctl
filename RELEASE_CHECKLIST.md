# Release Checklist

- [ ] Bump version in Cargo.toml (single source of truth)
- [ ] Update CHANGELOG.md
- [ ] Run `make test` and `make lint`
- [ ] Run `make package-deb` and `make package-rpm`
- [ ] Verify install/upgrade/uninstall in containers (Debian, Fedora, Arch)
- [ ] Sign artifacts with GPG and publish checksums
- [ ] Tag release (vX.Y.Z) and push
