# il2cpp

Android native library to dump Unity IL2CPP game metadata at runtime.

## How it works

1. Library is loaded into a Unity Android app (via JNI `JNI_OnLoad`)
2. Spawns a thread that waits for `libil2cpp.so` to be loaded
3. Uses [xDL](https://github.com/hexhacking/xDL) to open `libil2cpp.so` and resolve all IL2CPP API functions
4. Enumerates all assemblies, classes, fields, properties, and methods
5. Writes `dump.cs` to `Application.persistentDataPath`

## Features

- Dumps full class hierarchy with namespaces, base types, and interfaces
- Method signatures with return types, parameters, and access modifiers
- Field types, offsets, and constant values
- Property getters/setters
- Supports Unity 2018.3+ (image API) and older versions (reflection fallback)
- Also provides `UnityResolve.hpp` – C++ wrapper for runtime Unity object manipulation

## Build

Requires Android NDK r26d+.

```bash
mkdir build && cd build
cmake .. \
  -DCMAKE_TOOLCHAIN_FILE=$NDK/build/cmake/android.toolchain.cmake \
  -DANDROID_ABI=arm64-v8a \
  -DANDROID_PLATFORM=android-24 \
  -DANDROID_STL=c++_static
make -j$(nproc)
```

Output: `libDemo.so`

## Usage

- Inject `libDemo.so` into a Unity IL2CPP Android app
- `dump.cs` will be written to the app's persistent data path

## Dependencies

- [xDL](https://github.com/hexhacking/xDL) – enhanced dynamic linker for Android
