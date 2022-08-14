package main

import (
	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()
	router.SetTrustedProxies([]string{})
	router.Static("/", "./static")
	router.Run()
}
