VERSION=0.0.0

RUSTC=rustc
COPTS=--crate-type bin --crate-name rshui
REL_COPTS=-C opt-level=s -C panic=abort -C overflow-checks=no

rshui: rshui.rs menu.rs config.rs
	$(RUSTC) $(COPTS) $(REL_COPTS) -o $@ $<

d_rshui: rshui.rs menu.rs config.rs
	$(RUSTC) $(COPTS) -g -o $@ $<

rshui.s: rshui.rs menu.rs config.rs
	$(RUSTC) $(COPTS) $(REL_COPTS) -o $@ --emit asm $<

clean:
	rm -f hui
	rm -f d_hui
	rm -f rshui.s
