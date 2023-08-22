package main

import (
	"os"

	"github.com/aymerick/raymond"
	"github.com/gofiber/fiber/v2"
)

func renderTemplate(name string) (string, error) {
	rawTemplate, err := os.ReadFile(name)
	if err != nil {
		return "", err
	}
	template := string(rawTemplate)

	ctx := map[string]string{
		"title":   "Hello, World!",
		"content": "Hello, World!",
	}
	result, err := raymond.Render(template, ctx)
	if err != nil {
		return template, err
	}

	return result, nil
}

func main() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		content, err := renderTemplate("templates/main.html")
		if err != nil {
			return err
		}
		c.Set(fiber.HeaderContentType, fiber.MIMETextHTML)
		return c.SendString(content)
	})
	app.Get("/favicon.ico", func(c *fiber.Ctx) error {
		return c.SendFile("static/favicon.ico", true)
	})
	app.Get("/style.css", func(c *fiber.Ctx) error {
		return c.SendFile("static/style.css", true)
	})

	port := os.Getenv("PORT")
	if port == "" {
		port = "3000"
	}
	app.Listen(":" + port)
}
