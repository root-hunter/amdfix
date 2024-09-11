boost-activate:
	cd target/release; ./amdfix boost active

boost-deactivate:
	cd target/release; ./amdfix boost deactive

build-release:
	cargo build --release