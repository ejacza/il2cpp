#include <jni.h>
#include <pthread.h>
#include <unistd.h>
#include "il2cpp/log.h"
#include "il2cpp/il2cpp_dump.h"
#include "il2cpp/UnityResolve.hpp"
#include "xdl/include/xdl.h"

JavaVM *g_vm = nullptr;

void *hack_thread(void *) {
    bool load = false;
    for (int i = 0; i < 10; i++) {
        void *handle = xdl_open("libil2cpp.so", 0);
        if (handle) {
            load = true;
            il2cpp_api_init(handle);
            il2cpp_dump();
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
