// seedfaker — deterministic synthetic data generator
//
// Install:  go get github.com/opendsr-std/seedfaker-go
// Requires: libseedfaker_ffi shared library, CGO enabled
// Docs:     https://github.com/opendsr-std/seedfaker

package main

/*
#cgo LDFLAGS: -lseedfaker_ffi
#include "seedfaker.h"
#include <stdlib.h>
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"
)

func main() {
	// Deterministic: same seed = same output, always
	opts := `{"seed":"demo","locale":"en"}`
	cOpts := C.CString(opts)
	defer C.free(unsafe.Pointer(cOpts))
	handle := C.sf_create(cOpts)
	defer C.sf_destroy(handle)

	// Single fields
	for _, field := range []string{"name", "email", "phone"} {
		cf := C.CString(field)
		ptr := C.sf_field(handle, cf)
		C.free(unsafe.Pointer(cf))
		fmt.Printf("%s: %s\n", field, C.GoString(ptr))
		C.sf_free(ptr)
	}

	// Correlated records: email derived from name, phone matches locale
	bulk := `{"fields":["name","email","phone"],"n":5,"ctx":"strict"}`
	cBulk := C.CString(bulk)
	ptr := C.sf_records(handle, cBulk)
	C.free(unsafe.Pointer(cBulk))
	var records []map[string]string
	json.Unmarshal([]byte(C.GoString(ptr)), &records)
	C.sf_free(ptr)

	fmt.Println("\nRecords (ctx=strict):")
	for _, r := range records {
		fmt.Printf("  %s\t%s\t%s\n", r["name"], r["email"], r["phone"])
	}

	// Fingerprint
	fpPtr := C.sf_fingerprint()
	fmt.Println("\nfingerprint:", C.GoString(fpPtr))
	C.sf_free(fpPtr)

	// Verify determinism
	seedA := C.CString(`{"seed":"ci"}`)
	seedB := C.CString(`{"seed":"ci"}`)
	hA := C.sf_create(seedA)
	hB := C.sf_create(seedB)
	C.free(unsafe.Pointer(seedA))
	C.free(unsafe.Pointer(seedB))
	cName := C.CString("name")
	pA := C.sf_field(hA, cName)
	pB := C.sf_field(hB, cName)
	C.free(unsafe.Pointer(cName))
	if C.GoString(pA) != C.GoString(pB) {
		panic("determinism failed")
	}
	C.sf_free(pA)
	C.sf_free(pB)
	C.sf_destroy(hA)
	C.sf_destroy(hB)
	fmt.Println("determinism: ok")
}
