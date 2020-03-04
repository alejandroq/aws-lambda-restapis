package main

import "github.com/alejandroq/aws-lambda-restapis/internal/router"

func main() {
	router.Get().Run()
}
