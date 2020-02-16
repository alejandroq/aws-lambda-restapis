package main

import (
	"github.com/akrylysov/algnhsa"
	"github.com/alejandroq/Moc/transactionsapi/internal/handlers/router"
)

func main() {
	algnhsa.ListenAndServe(router.Get(), nil)
}
