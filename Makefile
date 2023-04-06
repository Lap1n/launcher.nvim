build_linux:
	cargo build && cp target/debug/liblauncher.so lua/launcher.so
build_ios:
	cargo build && cp target/debug/liblauncher.dylib lua/launcher.so
build_window::
	cargo build && cp target/debug/launcher.dll lua/launcher.dll
release_linux:
	cargo build --release && cp target/release/liblauncher.so lua/launcher.so
release_ios:
	cargo build --release && cp target/release/liblauncher.dylib lua/launcher.so
release_window::
	cargo build --release && cp target/release/launcher.dll lua/launcher.dll
