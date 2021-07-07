#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <stdbool.h>

void* roc_alloc(size_t size, unsigned int alignment) {
    return malloc(size);
}

void* roc_realloc(void* ptr, size_t old_size, size_t new_size, unsigned int alignment) {
    return realloc(ptr, new_size);
}

void roc_dealloc(void* ptr, unsigned int alignment) {
    free(ptr);
}

struct RocStr {
    char* bytes;
    size_t len;
};

bool is_small_str(struct RocStr str) {
    return ((ssize_t)str.len) < 0;
}

// Determine the length of the string, taking into
// account the small string optimization
size_t roc_str_len(struct RocStr str) {
    char* bytes = (char*)&str;
    char last_byte = bytes[sizeof(str) - 1];
    char last_byte_xored = last_byte ^ 0b10000000;
    size_t small_len = (size_t)(last_byte_xored);
    size_t big_len = str.len;

    // Avoid branch misprediction costs by always
    // determining both small_len and big_len,
    // so this compiles to a cmov instruction.
    if (is_small_str(str)) {
        return small_len;
    } else {
        return big_len;
    }
}

struct RocCallResult {
    size_t flag;
    struct RocStr content;
};

extern void roc__mainForHost_1_exposed(struct RocCallResult *re);

int main() {
    // Make space for the Roc call result
    struct RocCallResult call_result;

    // Call Roc to populate call_result
    roc__mainForHost_1_exposed(&call_result);

    // Determine str_len and the str_bytes pointer,
    // taking into account the small string optimization.
    struct RocStr str = call_result.content;
    size_t str_len = roc_str_len(str);
    char* str_bytes;

    if (is_small_str(str)) {
        str_bytes = (char*)&str;
    } else {
        str_bytes = str.bytes;
    }

    // Write to stdout
    if (write(1, str_bytes, str_len) >= 0) {
        // Writing succeeded!
        return 0;
    } else {
        printf("Error writing to stdout: %s\n", strerror(errno));

        return 1;
    }
}