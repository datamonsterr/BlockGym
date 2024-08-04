package main

import (
	"log"

	"github.com/datamonsterr/PTSolana/handler"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func main() {
	e := echo.New()

	e.Use(middleware.Recover())
	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "${time_rfc3339} ${method} ${status} ${uri} ${error}\n",
	}))

	e.GET("/", handler.GetIndex)
	e.GET("/get-main-view", handler.GetMainView)
	e.GET("/user/:id", handler.GetUser)

	e.Static("/static", "static")
	log.Fatal(e.Start(":8080"))
}
