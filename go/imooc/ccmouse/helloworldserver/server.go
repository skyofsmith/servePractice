package main

import (
    "fmt"
    "net/http"
)

func main() {
    http.HandleFunc("/", func(
        writer http.ResponseWriter,
        request *http.Request)  {
          fmt.Fprintln(writer,
            "<h1>Hello world!</h1>")
    })

    http.ListenAndServe(":8888", nil)
}
