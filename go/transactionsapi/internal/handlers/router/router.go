package router

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

//Get an application router
func Get() *gin.Engine {
	r := gin.Default()
	r.GET("/", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "hello, world!",
		})
	})
	r.GET("/hello", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "goodbye, world!",
		})
	})
	return r
}
