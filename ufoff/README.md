# Using https://github.com/googlefonts/OswaldFont

```
cargo run \
	~/oss/OswaldFont/legacy/3.0/Roman/400/src/Oswald--400.ufo \
	/tmp/oswald.bin

# writes /tmp/json/oswald.json
flatc --json --raw-binary -o /tmp/json ../flatfont.fbs -- /tmp/oswald.bin
```

## Update generated code

```
flatc --rust flatfont.fbs
mv flatfont_generated.rs ufoff/src/
```