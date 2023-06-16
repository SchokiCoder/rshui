APP_NAME = rshui
VERSION = 0.0.0

RUSTC = rustc
COPTS = --crate-type bin --crate-name ${APP_NAME}
REL_COPTS = -C opt-level=s -C panic=abort -C overflow-checks=no

release:
	$(RUSTC) ${COPTS} ${REL_COPTS} -o ${APP_NAME} rshui.rs

debug:
	$(RUSTC) ${COPTS} -o ${APP_NAME} -g rshui.rs

asm:
	$(RUSTC) ${COPTS} ${REL_COPTS} -o ${APP_NAME}.asm --emit asm rshui.rs

clean:
	rm -f ${APP_NAME}
