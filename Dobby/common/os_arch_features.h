#pragma once

#include <sys/types.h>
#include <stddef.h>
#include "pac_kit.h"

#include "PlatformUnifiedInterface/platform.h"

#if defined(ANDROID)
#include <sys/mman.h>
#include <unistd.h>
#endif

#if defined(__arm64e__) && __has_feature(ptrauth_calls)
#include <ptrauth.h>
#endif

namespace features {

template <typename T> inline T arm_thumb_fix_addr(T &addr) {
#if defined(__arm__) || defined(__aarch64__)
  addr = (T)((uintptr_t)addr & ~1);
#endif
  return addr;
}

namespace apple {
template <typename T> inline T arm64e_pac_strip(T &addr) {
  return pac_strip(addr);
}

template <typename T> inline T arm64e_pac_sign(T &addr) {
  return pac_sign(addr);
}

template <typename T> inline T arm64e_pac_strip_and_sign(T &addr) {
  return pac_strip_and_sign(addr);
}
} // namespace apple

namespace android {
inline void make_memory_readable(void *address, size_t size) {
#if defined(ANDROID)
  // NOTE: use mprotect directly instead of OSMemory here: this header is
  // pulled in (via dobby/common.h) before platform.h finishes declaring
  // OSMemory/MemoryPermission, so referencing them would not compile.
  size_t page_size = (size_t)sysconf(_SC_PAGESIZE);
  uintptr_t start = (uintptr_t)address & ~(page_size - 1);
  uintptr_t end = ((uintptr_t)address + size + page_size - 1) & ~(page_size - 1);
  mprotect((void *)start, end - start, PROT_READ | PROT_EXEC);
#endif
}
} // namespace android
} // namespace features