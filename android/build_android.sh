cargo ndk  -t arm64-v8a -o ./jniLibs build --release
cp -rf ../assets android_studio/app/src/main 
cp -rf jniLibs android_studio/app/src/main