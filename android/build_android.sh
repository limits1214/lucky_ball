# cargo ndk -t arm64-v8a -o ./jniLibs build --release
# cargo ndk  -t arm64-v8a -o ./jniLibs build --release

cargo ndk  -t arm64-v8a \
    -t armeabi-v7a \
    -t x86 \
    -t x86_64 \
    -o ./jniLibs build --release

cp -rf ../assets android_studio/app/src/main 
cp -rf jniLibs android_studio/app/src/main