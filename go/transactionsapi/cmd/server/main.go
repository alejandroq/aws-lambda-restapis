package main

import "github.com/alejandroq/aws-lambda-restapis/internal/handlers/router"

func main() {
	router.Get().Run()
}
