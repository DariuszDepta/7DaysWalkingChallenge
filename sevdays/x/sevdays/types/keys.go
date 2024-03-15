package types

const (
	// ModuleName defines the module name
	ModuleName = "sevdays"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_sevdays"
)

var (
	ParamsKey = []byte("p_sevdays")
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}
