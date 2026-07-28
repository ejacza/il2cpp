
#include <jni.h>
#include <pthread.h>
#include <unistd.h>
#include "xdl/include/xdl.h"
#include <android/log.h>

JavaVM *g_vm = nullptr;

extern "C" {
    void rust_il2cpp_api_init(void *handle);
    void rust_il2cpp_dump();
}

void *hack_thread(void *) {
    bool load = false;
    for (int i = 0; i < 10; i++) {
        void *handle = xdl_open("libil2cpp.so", 0);
        if (handle) {
            load = true;
            rust_il2cpp_api_init(handle);
            rust_il2cpp_dump();
            break;
        } else {
            sleep(1);
        }
    }
    if (!load) {
        __android_log_print(ANDROID_LOG_INFO, "Demo", "libil2cpp.so not found in thread %d", gettid());
    }
    return nullptr;
}

JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM *vm, void *) {
    g_vm = vm;
    pthread_t thread;
    pthread_create(&thread, nullptr, hack_thread, nullptr);
    pthread_detach(thread);
    return JNI_VERSION_1_6;
}
