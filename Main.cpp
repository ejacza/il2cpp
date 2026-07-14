#include <jni.h>
#include <pthread.h>
#include <unistd.h>
#include <string>
#include <vector>
#include "il2cpp/log.h"
#include "il2cpp/il2cpp_dump.h"
#include "il2cpp/UnityResolve.hpp"
#include "xdl/include/xdl.h"

JavaVM *g_vm = nullptr;

// ── Compile-check: contoh pemakaian UnityResolve.hpp ──────────────
// Dipanggil setelah il2cpp_api_init(), jadi API function pointers sudah siap.
static void example_unityresolve() {
    using namespace UnityResolve;

    LOGI("=== UnityResolve example ===");

    // 1. Cari assembly
    auto assembly = Get("Assembly-CSharp.dll");
    if (!assembly) {
        LOGI("Assembly-CSharp.dll not found, skipping examples");
        return;
    }
    LOGI("Assembly found: %s", assembly->name.c_str());

    // 2. Cari class
    auto klass = assembly->Get("Player", "GameLogic");
    if (!klass) {
        LOGI("Class Player not found, skipping field/method examples");
    } else {
        LOGI("Class found: %s::%s", klass->namespaze.c_str(), klass->name.c_str());

        // 3. Field lookup
        auto field = klass->Get<Field>("health");
        if (field) {
            LOGI("Field found: %s (offset %d)", field->name.c_str(), field->offset);
        }

        // 4. Method lookup
        auto method = klass->Get<Method>("TakeDamage");
        if (method) {
            LOGI("Method found: %s", method->name.c_str());
        }
    }

    // 5. Managed String
    auto str = UnityType::String::New("hello from il2cpp");
    if (str) {
        LOGI("String created: %s", str->ToString().c_str());
    }

    // 6. GameObject / Transform
    auto playerObj = UnityType::GameObject::Find("Player");
    if (playerObj) {
        LOGI("GameObject found: Player");
        auto transform = playerObj->GetTransform();
        if (transform) {
            auto pos = transform->GetPosition();
            LOGI("Position: %.2f %.2f %.2f", pos.x, pos.y, pos.z);
        }
    }

    // 7. Camera
    auto cam = UnityType::Camera::GetMain();
    if (cam) {
        auto fov = cam->GetFoV();
        LOGI("Main camera FoV: %.2f", fov);
    }

    // 8. Time
    auto dt = UnityType::Time::GetDeltaTime();
    auto ts = UnityType::Time::GetTimeScale();
    LOGI("DeltaTime: %.4f  TimeScale: %.2f", dt, ts);

    // 9. Application
    auto dataPath = UnityType::Application::get_persistentDataPath();
    if (dataPath) {
        LOGI("PersistentDataPath: %s", dataPath->ToString().c_str());
    }

    // 10. Array template
    auto intClass = Get("mscorlib.dll")->Get("Int32", "System");
    if (intClass) {
        auto arr = UnityType::Array<int>::New(intClass, 5);
        if (arr) {
            arr->At(0) = 42;
            arr->At(1) = 99;
            LOGI("Array created, length: %zu, [0]: %d", arr->max_length, arr->At(0));
        }
    }

    LOGI("=== UnityResolve example done ===");
}

void *hack_thread(void *) {
    bool load = false;
    for (int i = 0; i < 10; i++) {
        void *handle = xdl_open("libil2cpp.so", 0);
        if (handle) {
            load = true;
            il2cpp_api_init(handle);
            il2cpp_dump();
            example_unityresolve();          // jalankan contoh
            break;
        } else {
            sleep(1);
        }
    }
    if (!load) {
        LOGI("libil2cpp.so not found in thread %d", gettid());
    }
    return nullptr;
}

JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM *vm, void * /*reserved*/) {
    g_vm = vm;

    pthread_t thread;
    pthread_create(&thread, nullptr, hack_thread, nullptr);
    pthread_detach(thread);

    return JNI_VERSION_1_6;
}
