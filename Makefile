check_rust:
	chmod +x check.sh
	bash check.sh
install:
	cargo build --release
	chmod +x ./target/release/watchmon
	cp ./target/release/watchmon /usr/local/bin/
