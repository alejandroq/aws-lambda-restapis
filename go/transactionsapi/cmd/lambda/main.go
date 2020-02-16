package main

import (
	"github.com/akrylysov/algnhsa"
	"github.com/alejandroq/aws-lambda-restapis/internal/handlers/router"
)

func main() {
	algnhsa.ListenAndServe(router.Get(), nil)
}
