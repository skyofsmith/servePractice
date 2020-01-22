package main

import (
	"log"
	"net/http"

	"apiserver/router"

	"github.com/gin-gonic/gin"
)

func main() {
	// Create the Gin engine.
	g := gin.New()

	// gin middlewares
	middlewares := []gin.HandlerFunc{}

	// Routes.
	router.Load(
		// Cores.
		g,

		// Middlewares.
		middlewares...,
	)

	log.Printf("Start to listening the incoming requests on http address: %s", ":8080")
	log.Printf(http.ListenAndServe(":8080", g).Error())
}