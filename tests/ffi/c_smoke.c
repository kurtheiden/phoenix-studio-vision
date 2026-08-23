#include "phoenix.h"

#include <stdio.h>
#include <string.h>

static int contains_bytes(const unsigned char *bytes, size_t length,
                          const char *needle) {
    size_t needle_length = strlen(needle);
    if (needle_length == 0 || needle_length > length) {
        return 0;
    }
    for (size_t offset = 0; offset + needle_length <= length; ++offset) {
        if (memcmp(bytes + offset, needle, needle_length) == 0) {
            return 1;
        }
    }
    return 0;
}

static int fail(const char *message) {
    fprintf(stderr, "phoenix UI0G1 C smoke failure: %s\n", message);
    return 1;
}

int main(void) {
    const unsigned char request[] =
        "{\"operation\":\"get_api_info\",\"payload\":{}}";
    phoenix_service_handle_t handle = 0;
    phoenix_buffer_t response = {0};
    int status;

    if (phoenix_abi_version() != 1) {
        return fail("unexpected ABI version");
    }
    status = phoenix_service_create(&handle);
    if (status != PHOENIX_STATUS_OK || handle == 0) {
        return fail("service creation failed");
    }

    status = phoenix_call(handle, request, sizeof(request) - 1, &response);
    if (status != PHOENIX_STATUS_OK || response.ptr == NULL || response.len == 0) {
        phoenix_service_destroy(handle);
        return fail("get_api_info call did not return a response");
    }
    if (!contains_bytes(response.ptr, response.len, "\"ok\":true") ||
        !contains_bytes(response.ptr, response.len, "\"contract_version\":1")) {
        phoenix_free_buffer(response);
        phoenix_service_destroy(handle);
        return fail("get_api_info response markers were not found");
    }
    if (phoenix_free_buffer(response) != PHOENIX_STATUS_OK) {
        phoenix_service_destroy(handle);
        return fail("response buffer release failed");
    }
    response.ptr = NULL;
    response.len = 0;

    if (phoenix_service_destroy(handle) != PHOENIX_STATUS_OK) {
        return fail("service destruction failed");
    }

    status = phoenix_call(handle, request, sizeof(request) - 1, &response);
    if (status != PHOENIX_STATUS_OK || response.ptr == NULL || response.len == 0) {
        return fail("stale-handle call did not return JSON");
    }
    if (!contains_bytes(response.ptr, response.len, "\"kind\":\"transport\"") ||
        !contains_bytes(response.ptr, response.len, "\"code\":\"invalid_handle\"")) {
        phoenix_free_buffer(response);
        return fail("stale-handle response was not invalid_handle transport JSON");
    }
    if (phoenix_free_buffer(response) != PHOENIX_STATUS_OK) {
        return fail("stale-handle response release failed");
    }
    if (phoenix_service_destroy(handle) != PHOENIX_STATUS_INVALID_HANDLE) {
        return fail("second destroy did not return invalid-handle status");
    }

    puts("UI0G1 external C bridge smoke passed");
    return 0;
}
