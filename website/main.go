package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/nanmu42/gzip"
)

func AddRoutes(router *gin.Engine) {
	router.GET("/", func(c *gin.Context) {
		c.HTML(http.StatusOK, "root/index.tmpl", gin.H{})
	})
	router.GET("/about", func(c *gin.Context) {
		c.HTML(http.StatusOK, "root/about.tmpl", gin.H{})
	})
	router.GET("/writing", func(c *gin.Context) {
		c.HTML(http.StatusOK, "root/writing.tmpl", gin.H{})
	})
	router.GET("/writing/cs_pedagogy_0", func(c *gin.Context) {
		c.HTML(http.StatusOK, "writing/cs_pedagogy_0.tmpl", gin.H{})
	})
	router.GET("/writing/programming_books", func(c *gin.Context) {
		c.HTML(http.StatusOK, "writing/programming_books.tmpl", gin.H{})
	})
	router.GET("/writing/learning_rust", func(c *gin.Context) {
		c.HTML(http.StatusOK, "writing/learning_rust.tmpl", gin.H{})
	})
}

func main() {
	router := gin.Default()
	router.Use(gzip.DefaultHandler().Gin)
	router.SetTrustedProxies([]string{})
	router.LoadHTMLGlob("templates/*/*.html")
	AddRoutes(router)
	router.Run()
}
