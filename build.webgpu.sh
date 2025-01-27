name="web"
flag="release" # debug or release

# rm -rf wasm

# RUSTFLAGS="--cfg=web_sys_unstable_apis" 
# --no-default-features
cargo build $( [ "$flag" = "release" ] && echo "--release" ) --target wasm32-unknown-unknown --features webgpu
wasm-bindgen --out-name $name \
  --out-dir wasm_webgpu \
  --target web ../target/wasm32-unknown-unknown/${flag}/$name.wasm

# sed -i '' 's/module_or_path = fetch(module_or_path)/module_or_path = bevyProgressiveFetch(module_or_path)/' ./wasm/${name}.js

wasm-opt -Oz --output optimized.wasm wasm_webgpu/${name}_bg.wasm
mv optimized.wasm wasm_webgpu/${name}_bg.wasm

# brotli wasm/${name}_bg.wasm -o wasm/${name}_bg.wasm.br
# gzip -c wasm/${name}_bg.wasm > wasm/${name}_bg.wasm.gz
# zstd wasm/${name}_bg.wasm -o wasm/${name}_bg.wasm.zst

# cp -r wasm ../../server/static