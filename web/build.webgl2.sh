name="web"
flag="release" # debug or release

# rm -rf wasm

# RUSTFLAGS="--cfg=web_sys_unstable_apis" 
# --no-default-features
cargo build $( [ "$flag" = "release" ] && echo "--release" ) --target wasm32-unknown-unknown --features webgl2
wasm-bindgen --out-name $name \
  --out-dir wasm_webgl2 \
  --target web ../target/wasm32-unknown-unknown/${flag}/$name.wasm

# sed -i '' 's/module_or_path = fetch(module_or_path)/module_or_path = bevyProgressiveFetch(module_or_path)/' ./wasm/${name}.js

if [ "$(uname)" = "Darwin" ]; then
  sed -i '' 's/module_or_path = fetch(module_or_path)/module_or_path = bevyProgressiveFetch(module_or_path)/' wasm_webgl2/${name}.js
else
  sed -i 's/module_or_path = fetch(module_or_path)/module_or_path = bevyProgressiveFetch(module_or_path)/' wasm_webgl2/${name}.js
fi

wasm-opt -Oz --output optimized.wasm wasm_webgl2/${name}_bg.wasm
mv optimized.wasm wasm_webgl2/${name}_bg.wasm

# brotli wasm/${name}_bg.wasm -o wasm/${name}_bg.wasm.br
# gzip -c wasm/${name}_bg.wasm > wasm/${name}_bg.wasm.gz
# zstd wasm/${name}_bg.wasm -o wasm/${name}_bg.wasm.zst

# cp -r wasm ../../server/static
echo $(stat -f %z wasm_webgl2/${name}_bg.wasm)