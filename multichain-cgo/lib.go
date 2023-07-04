package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"unsafe"

	keygen "github.com/anyswap/FastMulThreshold-DSA/smpc-lib/ecdsa/keygen"
	"github.com/anyswap/FastMulThreshold-DSA/smpc-lib/smpc"
	// signing "github.com/anyswap/FastMulThreshold-DSA/smpc-lib/ecdsa/signing"
)

//export CreateNode
func CreateNode(count int, group int, pallier_key_length int, keyType *C.char) *C.char {
	var out chan smpc.Message
	var end chan keygen.LocalDNodeSaveData
	out = make(chan smpc.Message, 1000)
	end = make(chan keygen.LocalDNodeSaveData, 1000)
	dnode := keygen.NewLocalDNode(out, end, count, group, pallier_key_length, C.GoString(keyType))
	firstRound := dnode.FirstRound()
	roundResult := firstRound.Start()
	if roundResult != nil {
		return C.CString(roundResult.Error())
	}
	return C.CString("OK")
}

//export GoFree
func GoFree(ptr *C.char) {
	C.free(unsafe.Pointer(ptr))
}

func main() {
}
