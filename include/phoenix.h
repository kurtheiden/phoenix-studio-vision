#ifndef PHOENIX_H
#define PHOENIX_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define PHOENIX_ABI_VERSION UINT32_C(1)

#define PHOENIX_STATUS_OK INT32_C(0)
#define PHOENIX_STATUS_INVALID_ARGUMENT INT32_C(1)
#define PHOENIX_STATUS_INVALID_HANDLE INT32_C(2)
#define PHOENIX_STATUS_INTERNAL_FAILURE INT32_C(3)

typedef uint64_t phoenix_service_handle_t;

typedef struct phoenix_buffer {
    uint8_t *ptr;
    size_t len;
} phoenix_buffer_t;

/* Return the C ABI version. This is independent of the JSON contract version. */
uint32_t phoenix_abi_version(void);

/*
 * Create a process-local Phoenix service. out_handle must be non-NULL,
 * correctly aligned, writable for one phoenix_service_handle_t, and valid for
 * the complete call. It is zeroed before fallible work and receives a nonzero
 * handle only on success.
 */
int32_t phoenix_service_create(phoenix_service_handle_t *out_handle);

/*
 * Destroy a service and wait for all calls admitted before destruction.
 * Successful destruction permanently invalidates the handle. This operation
 * does not cancel work.
 */
int32_t phoenix_service_destroy(phoenix_service_handle_t handle);

/*
 * Synchronously dispatch request_len bytes of JSON through the service.
 * request_ptr is borrowed only for this call and is not NUL-terminated. When
 * request_len is nonzero, request_ptr must be non-NULL, correctly aligned,
 * readable for request_len bytes, and valid for the complete call.
 *
 * out_response must be non-NULL, correctly aligned, writable for one
 * phoenix_buffer_t, and valid for the complete call. Status OK means it owns a
 * nonempty UTF-8 JSON response allocated by Phoenix. Every nonzero status
 * leaves it zeroed and without a trustworthy response. Release a successful
 * response exactly once with phoenix_free_buffer; never use libc free or a
 * different allocator.
 */
int32_t phoenix_call(phoenix_service_handle_t handle,
                     const uint8_t *request_ptr,
                     size_t request_len,
                     phoenix_buffer_t *out_response);

/*
 * Release an unchanged buffer returned by phoenix_call. {NULL, 0} is a valid
 * no-op; {NULL, nonzero} is invalid. Fabricated pointers, modified lengths,
 * and double frees violate the API contract and cannot generally be detected.
 */
int32_t phoenix_free_buffer(phoenix_buffer_t buffer);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* PHOENIX_H */
