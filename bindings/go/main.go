package main

// #cgo CFLAGS: -g -Wall
// #include <stdlib.h>
// #include "multiply.h"
import "C"
import "fmt"

func main() {
	a := C.int(25)
	b := C.int(30)
	c := C.int(35)
	d := C.int(54)
	e := C.int(654)

	result := C.multiply(a, b, c, d, e)
	fmt.Println(int(result))
}
