// Package seedfaker provides deterministic synthetic data generation via FFI.
// Same seed + same arguments = identical output across CLI, Node, Python, PHP, Ruby, Go.
package seedfaker

/*
#cgo darwin,arm64 LDFLAGS: -L${SRCDIR}/bin/darwin-arm64 -Wl,-rpath,${SRCDIR}/bin/darwin-arm64
#cgo darwin,amd64 LDFLAGS: -L${SRCDIR}/bin/darwin-x86_64 -Wl,-rpath,${SRCDIR}/bin/darwin-x86_64
#cgo linux,amd64 LDFLAGS: -L${SRCDIR}/bin/linux-x86_64 -Wl,-rpath,${SRCDIR}/bin/linux-x86_64
#cgo linux,arm64 LDFLAGS: -L${SRCDIR}/bin/linux-arm64 -Wl,-rpath,${SRCDIR}/bin/linux-arm64
#cgo LDFLAGS: -lseedfaker_ffi
#include "seedfaker.h"
#include <stdlib.h>
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"runtime"
	"unsafe"
)

// SeedFaker generates deterministic synthetic data.
type SeedFaker struct {
	handle *C.SfFaker
}

// Options for creating a SeedFaker instance.
type Options struct {
	Seed   string `json:"seed,omitempty"`
	Locale string `json:"locale,omitempty"`
	Tz     string `json:"tz,omitempty"`
	Since  int    `json:"since,omitempty"`
	Until  int    `json:"until,omitempty"`
}

// RecordOpts for generating multiple records.
type RecordOpts struct {
	Fields  []string `json:"fields"`
	N       int      `json:"n"`
	Ctx     string   `json:"ctx,omitempty"`
	Corrupt string   `json:"corrupt,omitempty"`
}

// New creates a SeedFaker instance.
func New(opts Options) (*SeedFaker, error) {
	optsJSON, err := json.Marshal(opts)
	if err != nil {
		return nil, fmt.Errorf("seedfaker: marshal opts: %w", err)
	}
	cOpts := C.CString(string(optsJSON))
	defer C.free(unsafe.Pointer(cOpts))

	handle := C.sf_create(cOpts)
	if handle == nil {
		return nil, fmt.Errorf("seedfaker: sf_create: %s", C.GoString(C.sf_last_error()))
	}
	f := &SeedFaker{handle: handle}
	runtime.SetFinalizer(f, func(f *SeedFaker) { f.Close() })
	return f, nil
}

// Close releases the native handle.
func (f *SeedFaker) Close() {
	if f.handle != nil {
		C.sf_destroy(f.handle)
		f.handle = nil
	}
}

// Field returns field values. Use Opts.N > 1 for multiple values.
func (f *SeedFaker) Field(name string, opts ...Opts) ([]string, error) {
	var o Opts
	if len(opts) > 0 {
		o = opts[0]
	}
	spec := buildSpec(name, o)
	n := o.N
	if n < 1 {
		n = 1
	}
	vals := make([]string, 0, n)
	for i := 0; i < n; i++ {
		val, err := f.fieldOne(spec)
		if err != nil {
			return nil, err
		}
		vals = append(vals, val)
	}
	return vals, nil
}

func (f *SeedFaker) fieldOne(spec string) (string, error) {
	cField := C.CString(spec)
	defer C.free(unsafe.Pointer(cField))

	ptr := C.sf_field(f.handle, cField)
	if ptr == nil {
		return "", fmt.Errorf("seedfaker: sf_field(%s): %s", spec, C.GoString(C.sf_last_error()))
	}
	val := C.GoString(ptr)
	C.sf_free(ptr)
	return val, nil
}

// Record returns a single record as map[string]string.
func (f *SeedFaker) Record(fields []string, ctx, corrupt string) (map[string]string, error) {
	type recordOpts struct {
		Fields  []string `json:"fields"`
		Ctx     string   `json:"ctx,omitempty"`
		Corrupt string   `json:"corrupt,omitempty"`
	}
	optsJSON, err := json.Marshal(recordOpts{Fields: fields, Ctx: ctx, Corrupt: corrupt})
	if err != nil {
		return nil, fmt.Errorf("seedfaker: marshal opts: %w", err)
	}
	cOpts := C.CString(string(optsJSON))
	defer C.free(unsafe.Pointer(cOpts))

	ptr := C.sf_record(f.handle, cOpts)
	if ptr == nil {
		return nil, fmt.Errorf("seedfaker: sf_record: %s", C.GoString(C.sf_last_error()))
	}
	jsonStr := C.GoString(ptr)
	C.sf_free(ptr)

	var record map[string]string
	if err := json.Unmarshal([]byte(jsonStr), &record); err != nil {
		return nil, fmt.Errorf("seedfaker: unmarshal record: %w", err)
	}
	return record, nil
}

// Validate checks field specs without generating data. Returns error if invalid.
func (f *SeedFaker) Validate(fields []string, ctx, corrupt string) error {
	type validateOpts struct {
		Fields  []string `json:"fields"`
		Ctx     string   `json:"ctx,omitempty"`
		Corrupt string   `json:"corrupt,omitempty"`
	}
	optsJSON, err := json.Marshal(validateOpts{Fields: fields, Ctx: ctx, Corrupt: corrupt})
	if err != nil {
		return fmt.Errorf("seedfaker: marshal opts: %w", err)
	}
	cOpts := C.CString(string(optsJSON))
	defer C.free(unsafe.Pointer(cOpts))

	ptr := C.sf_validate(f.handle, cOpts)
	if ptr == nil {
		return fmt.Errorf("seedfaker: validate: %s", C.GoString(C.sf_last_error()))
	}
	C.sf_free(ptr)
	return nil
}

// Records returns records as []map[string]string.
func (f *SeedFaker) Records(opts RecordOpts) ([]map[string]string, error) {
	optsJSON, err := json.Marshal(opts)
	if err != nil {
		return nil, fmt.Errorf("seedfaker: marshal opts: %w", err)
	}
	cOpts := C.CString(string(optsJSON))
	defer C.free(unsafe.Pointer(cOpts))

	ptr := C.sf_records(f.handle, cOpts)
	if ptr == nil {
		return nil, fmt.Errorf("seedfaker: sf_records: %s", C.GoString(C.sf_last_error()))
	}
	jsonStr := C.GoString(ptr)
	C.sf_free(ptr)

	var records []map[string]string
	if err := json.Unmarshal([]byte(jsonStr), &records); err != nil {
		return nil, fmt.Errorf("seedfaker: unmarshal records: %w", err)
	}
	return records, nil
}

// Fields returns all available field names.
func Fields() ([]string, error) {
	ptr := C.sf_fields_json()
	if ptr == nil {
		return nil, fmt.Errorf("seedfaker: sf_fields_json failed")
	}
	jsonStr := C.GoString(ptr)
	C.sf_free(ptr)

	var fields []struct {
		Name string `json:"name"`
	}
	if err := json.Unmarshal([]byte(jsonStr), &fields); err != nil {
		return nil, err
	}
	names := make([]string, len(fields))
	for i, f := range fields {
		names[i] = f.Name
	}
	return names, nil
}

// Fingerprint returns the algorithm version identifier.
func Fingerprint() (string, error) {
	ptr := C.sf_fingerprint()
	if ptr == nil {
		return "", fmt.Errorf("seedfaker: sf_fingerprint failed")
	}
	val := C.GoString(ptr)
	C.sf_free(ptr)
	return val, nil
}
