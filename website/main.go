package main

import (
	"github.com/gin-contrib/secure"
	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()
	secureConfig := secure.DefaultConfig()
	secureConfig.ContentSecurityPolicy = "default-src 'self' etoast.me; style-src cdn.simplecss.org;"
	router.Use(secure.New(secureConfig))
	router.SetTrustedProxies([]string{})
	router.Static("/", "./static")
	router.Run()
}
