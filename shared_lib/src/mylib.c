#include "mylib.h"

#include <stdio.h>
#include <stdlib.h>

struct opaque_t {
    int field;
};

status_t mylib_create(handle_t* handle) {
    if (!handle) return FAIL;
    *handle = malloc(sizeof(struct opaque_t));
    return SUCCESS;
}

status_t mylib_destroy(handle_t handle) {
    if (!handle) return FAIL;
    free(handle);
    return SUCCESS;
}

status_t mylib_dump(handle_t handle) {
    if (!handle) return FAIL;
    printf("%d\n", handle->field);
    return SUCCESS;
}

status_t mylib_set(handle_t handle, int value) {
    if (!handle) return FAIL;
    handle->field = value;
    return SUCCESS;
}