boost-activate:
	cd target/release; ./amdfix boost active

boost-deactivate:
	cd target/release; ./amdfix boost deactive

boost-status:
	cd target/release; ./amdfix boost status

build-release:
	cargo build --release

install: build-release
	rm -f -r /usr/local/lib/amdfix/
	cp -r target/release /usr/local/lib/amdfix/
	rm -f /usr/bin/amdfix
	ln -s /usr/local/lib/amdfix/amdfix /usr/bin/amdfix