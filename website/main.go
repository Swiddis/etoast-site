package main

import (
	"log"
	"net/http"

	"github.com/gin-contrib/secure"
	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/acme/autocert"
)

func main() {
	router := gin.Default()
	secureConfig := secure.DefaultConfig()
	secureConfig.ContentSecurityPolicy = "default-src 'self' etoast.me; style-src cdn.simplecss.org;"
	router.Use(secure.New(secureConfig))
	router.SetTrustedProxies([]string{})
	router.Static("/", "./static")
	err := http.Serve(autocert.NewListener("etoast.me"), router)
	if err != nil {
		log.Fatal(err)
	}
}
