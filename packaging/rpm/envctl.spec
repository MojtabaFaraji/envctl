Name:           envctl
Version:        0.1.0
Release:        1%{?dist}
Summary:        Manage named shell environments
License:        MIT
URL:            https://github.com/example/envctl
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo, rust
Requires:       glibc

%description
envctl manages named shell environments stored as TOML profiles.

%prep
%setup -q

%build
cargo build --release

%install
install -Dm755 target/release/envctl %{buildroot}/usr/bin/envctl
install -Dm644 docs/envctl.1 %{buildroot}/usr/share/man/man1/envctl.1
install -Dm644 completions/envctl.bash %{buildroot}/usr/share/bash-completion/completions/envctl
install -Dm644 completions/_envctl %{buildroot}/usr/share/zsh/site-functions/_envctl
install -Dm644 completions/envctl.fish %{buildroot}/usr/share/fish/vendor_completions.d/envctl.fish
install -Dm644 LICENSE %{buildroot}/usr/share/licenses/envctl/LICENSE

%files
/usr/bin/envctl
/usr/share/man/man1/envctl.1
/usr/share/bash-completion/completions/envctl
/usr/share/zsh/site-functions/_envctl
/usr/share/fish/vendor_completions.d/envctl.fish
/usr/share/licenses/envctl/LICENSE

%changelog
* Mon Oct 27 2026 envctl contributors - 0.1.0-1
- Initial release