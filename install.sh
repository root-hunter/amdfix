make build-release
rm -f -r /usr/local/lib/amdfix/
cp -r target/release /usr/local/lib/amdfix/
rm -f /usr/bin/amdfix
ln -s /usr/local/lib/amdfix/amdfix /usr/bin/amdfix