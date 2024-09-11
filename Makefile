boost-activate:
	cd target/release; ./amdfix boost active

boost-deactivate:
	cd target/release; ./amdfix boost deactive

boost-status:
	cd target/release; ./amdfix boost status

build-release:
	cargo build --release