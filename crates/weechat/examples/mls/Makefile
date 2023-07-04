WEECHAT_HOME ?= $(HOME)/.weechat
PREFIX ?= $(WEECHAT_HOME)

.PHONY: install install-dir lint

target/debug/libmls.so: src/lib.rs
	cargo build

install: install-dir target/debug/libmls.so
	install -m644  target/debug/libmls.so $(DESTDIR)$(PREFIX)/plugins/mls.so

install-dir:
	install -d $(DESTDIR)$(PREFIX)/plugins

lint:
	cargo clippy
