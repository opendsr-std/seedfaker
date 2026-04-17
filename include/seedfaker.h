/* seedfaker C-ABI — load via FFI from PHP, Ruby, Go, C#, Java, etc.
 *
 * Memory contract:
 *   Every char* returned by sf_* must be freed with sf_free().
 *   sf_create returns an opaque handle; free with sf_destroy.
 *   sf_last_error returns a thread-local pointer — do NOT free it.
 *   sf_last_error pointer is valid until the next sf_* call on the same thread.
 *
 * Thread safety:
 *   SfFaker handles are NOT thread-safe. Do not share across threads.
 *   sf_fields_json, sf_fingerprint, sf_last_error are safe to call from any thread.
 */

#ifndef SEEDFAKER_H
#define SEEDFAKER_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SfFaker SfFaker;

/* Create instance. opts_json: {"seed":"x","locale":"en,de","tz":"+0300","since":1990,"until":2025}
 * All fields optional. NULL opts_json defaults to {}. Returns NULL on error (check sf_last_error). */
SfFaker* sf_create(const char* opts_json);

/* Destroy instance. Safe to call with NULL. */
void sf_destroy(SfFaker* faker);

/* Generate single field value. Supports modifier syntax: "phone:e164", "amount:usd".
 * Returns NULL on error. Caller must sf_free. */
char* sf_field(SfFaker* faker, const char* field_spec);

/* Validate field specs and options without generating data.
 * opts_json: {"fields":["name","email:e164"],"ctx":"strict","corrupt":"low"}
 * Returns empty string on success, NULL on error. Caller must sf_free. */
char* sf_validate(const SfFaker* faker, const char* opts_json);

/* Generate single record as JSON object.
 * opts_json: {"fields":["name","email"],"ctx":"strict","corrupt":"low"}
 * Returns NULL on error. Caller must sf_free. */
char* sf_record(SfFaker* faker, const char* opts_json);

/* Generate records as JSON array.
 * opts_json: {"fields":["name","email","phone:e164"],"n":100,"ctx":"strict","corrupt":"low"}
 * Fields support modifier syntax. Returns NULL on error. Caller must sf_free. */
char* sf_records(SfFaker* faker, const char* opts_json);

/* List all fields as JSON array. Caller must sf_free. */
char* sf_fields_json(void);

/* Algorithm fingerprint ("sf0-..."). Caller must sf_free. */
char* sf_fingerprint(void);

/* Build info JSON: {"version":"...","fingerprint":"..."}. Caller must sf_free. */
char* sf_build_info(void);

/* Free a string returned by any sf_* function. Safe to call with NULL. */
void sf_free(char* ptr);

/* Last error message. Thread-local, valid until next sf_* call on same thread. Do NOT free. */
const char* sf_last_error(void);

#ifdef __cplusplus
}
#endif

#endif /* SEEDFAKER_H */
