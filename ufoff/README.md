# Using https://github.com/googlefonts/OswaldFont

```
cargo run \
	~/oss/OswaldFont/legacy/3.0/Roman/400/src/Oswald--400.ufo \
	/tmp/oswald.bin

# writes /tmp/json/oswald.json
flatc --json --raw-binary -o /tmp/json ../schemas/ufo/fontinfo.fbs -- /tmp/oswald.bin \
	&& cat /tmp/json/oswald.json
```

## Update generated code

```
flatc --rust -o ufoff/src schemas/ufo/fontinfo.fbs 
```