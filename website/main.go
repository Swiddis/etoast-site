package main

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/acme/autocert"
)

func main() {
	router := gin.Default()
	router.SetTrustedProxies([]string{})
	router.Static("/", "./static")
	err := http.Serve(autocert.NewListener("etoast.me"), router)
	if err != nil {
		log.Fatal(err)
	}
}
