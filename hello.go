package main

import (
	"os"

	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendFile("content/index.html")
	})
	app.Get("/favicon.ico", func(c *fiber.Ctx) error {
		return c.SendFile("static/favicon.ico", true)
	})
	app.Get("/static/style.css", func(c *fiber.Ctx) error {
		return c.SendFile("static/style.css", true)
	})

	port := os.Getenv("PORT")
	if port == "" {
		port = "3000"
	}
	app.Listen(":" + port)
}
