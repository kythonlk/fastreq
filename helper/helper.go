package helper

import (
	"bytes"
	"encoding/json"
	"fmt"
	"github.com/kythonlk/fastreq/types"
	"net/http"
	"os"
)

func ReadConfig(filename string) (*types.Config, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	config := &types.Config{}
	err = json.NewDecoder(file).Decode(config)
	if err != nil {
		return nil, err
	}

	return config, nil
}

func SendRequest(config *types.Config) (*http.Response, error) {
	client := &http.Client{}
	req, err := http.NewRequest(config.Method, config.URL, bytes.NewBuffer([]byte(config.Body)))
	if err != nil {
		return nil, err
	}

	for key, value := range config.Headers {
		req.Header.Set(key, value)
	}

	return client.Do(req)
}

func FormatHeaders(headers http.Header) string {
	formattedHeaders := ""
	for key, values := range headers {
		for _, value := range values {
			formattedHeaders += fmt.Sprintf("%s: %s\n", key, value)
		}
	}
	return formattedHeaders
}

func PrintUsage() {
	fmt.Println("Usage:")
	fmt.Println("  go run main.go -F <config.json>")
	fmt.Println("  go run main.go -X <url> [-M <method>] [-H <header=value>]... [-B <body>]")
}
