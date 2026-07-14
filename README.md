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

Inject `libDemo.so` into a Unity IL2CPP Android app — `dump.cs` will be written to the app's persistent data path.

## UnityResolve.hpp

A high-level C++ wrapper for runtime Unity object manipulation via the IL2CPP API.

### Examples

**Get an assembly and a class:**
```cpp
auto assembly = UnityResolve::Get("Assembly-CSharp.dll");
auto klass = assembly->Get("Player", "GameLogic");
```

**Read/write a field on an object:**
```cpp
using Field = UnityResolve::Field;

auto healthField = klass->Get<Field>("health");
int hp = reinterpret_cast<int>(obj + healthField->offset);

// or using helper
int hp = klass->GetValue<int>(obj, "health");
klass->SetValue(obj, "health", 100);
```

**Invoke a method directly (native call):**
```cpp
using Method = UnityResolve::Method;

auto method = klass->Get<Method>("TakeDamage");
method->Invoke<void, int>(obj, 10);
```

**Invoke via runtime (with boxing/unboxing):**
```cpp
auto method = klass->Get<Method>("GetName");
auto result = method->RuntimeInvoke<String*>(obj);  // String = UnityResolve::UnityType::String
```

**Cast a method to a C++ function pointer:**
```cpp
using FP = void(*)(void*, int);
auto fp = method->Cast<void, void*, int>();
fp(obj, 10);
```

**Find Unity objects:**
```cpp
using UT = UnityResolve::UnityType;

auto playerObj = UT::GameObject::Find("Player");
auto transform = playerObj->GetTransform();
UT::Vector3 pos = transform->GetPosition();
```

**Access main camera:**
```cpp
auto cam = UT::Camera::GetMain();
float fov = cam->GetFoV();
UT::Vector3 screenPos = cam->WorldToScreenPoint(worldPos);
```

**Read/write static fields:**
```cpp
auto field = klass->Get<Field>("instance");
// use il2cpp_field_static_get_value / il2cpp_field_static_set_value directly
```

**Create managed String and Array:**
```cpp
auto str = UT::String::New("hello");
auto arr = UT::Array<int>::New(someClass, 10);
int val = arr->At(0);
```

**Iterate a managed List:**
```cpp
auto list = klass->GetValue<UT::List<Enemy*>*>(obj, "enemies");
for (auto& enemy : list->fields->items->ToVector()) {
    // use enemy
}
```

**Time and Application:**
```cpp
float dt = UT::Time::GetDeltaTime();
float time = UT::Time::GetTime();
auto dataPath = UT::Application::get_persistentDataPath();
```

## Dependencies

- [xDL](https://github.com/hexhacking/xDL) – enhanced dynamic linker for Android
