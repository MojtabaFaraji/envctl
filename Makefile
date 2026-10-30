PREFIX ?= /usr
DESTDIR ?=
CARGO ?= cargo
BINARY = target/release/envctl

.PHONY: build test lint install uninstall clean package-deb package-rpm package-aur

build:
	$(CARGO) build --release

test:
	$(CARGO) test

lint:
	$(CARGO) clippy -- -D warnings
	$(CARGO) fmt --check

install: build
	install -Dm755 $(BINARY) $(DESTDIR)$(PREFIX)/bin/envctl
	install -Dm644 docs/envctl.1 $(DESTDIR)$(PREFIX)/share/man/man1/envctl.1
	install -Dm644 completions/envctl.bash $(DESTDIR)$(PREFIX)/share/bash-completion/completions/envctl
	install -Dm644 completions/_envctl $(DESTDIR)$(PREFIX)/share/zsh/site-functions/_envctl
	install -Dm644 completions/envctl.fish $(DESTDIR)$(PREFIX)/share/fish/vendor_completions.d/envctl.fish
	install -Dm644 LICENSE $(DESTDIR)$(PREFIX)/share/licenses/envctl/LICENSE

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/envctl
	rm -f $(DESTDIR)$(PREFIX)/share/man/man1/envctl.1
	rm -f $(DESTDIR)$(PREFIX)/share/bash-completion/completions/envctl
	rm -f $(DESTDIR)$(PREFIX)/share/zsh/site-functions/_envctl
	rm -f $(DESTDIR)$(PREFIX)/share/fish/vendor_completions.d/envctl.fish
	rm -rf $(DESTDIR)$(PREFIX)/share/licenses/envctl

clean:
	$(CARGO) clean
	rm -rf dist packaging/deb/envctl packaging/rpm/BUILD*

# ---- Debian package ----
package-deb: build
	rm -rf dist/deb && mkdir -p dist/deb/envctl/DEBIAN
	cp packaging/deb/control dist/deb/envctl/DEBIAN/control
	sed -i 's/^Version:.*/Version: 0.1.0/' dist/deb/envctl/DEBIAN/control
	$(MAKE) install DESTDIR=dist/deb/envctl PREFIX=/usr
	dpkg-deb --build dist/deb/envctl dist/envctl_0.1.0_amd64.deb
	@echo "built dist/envctl_0.1.0_amd64.deb"

# ---- RPM package ----
package-rpm: build
	@if ! command -v rpmbuild >/dev/null 2>&1; then echo "rpmbuild not found; install rpm-build"; exit 1; fi
	mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
	cp packaging/rpm/envctl.spec ~/rpmbuild/SPECS/
	rpmbuild -bb ~/rpmbuild/SPECS/envctl.spec --define "_topdir $(HOME)/rpmbuild"
	@echo "built rpm in ~/rpmbuild/RPMS"

# ---- AUR ----
package-aur:
	@echo "See packaging/aur/PKGBUILD"