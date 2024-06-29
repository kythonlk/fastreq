package helper

import (
	"bytes"
	"encoding/json"
	"fmt"
	"github.com/kythonlk/fastreq/types"
	"net/http"
	"net/url"
	"os"
	"strings"
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

func InitializeConfig(filename string) error {
	config := &types.Config{
		BaseURL: "https://example.com",
		Method:  "GET",
		Headers: map[string]string{"Content-Type": "application/json"},
		Body:    "",
	}

	file, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	encoder := json.NewEncoder(file)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(config); err != nil {
		return err
	}

	fmt.Printf("Initialized new config file: %s\n", filename)
	return nil
}

func ConstructFullURL(baseURL, endpoint string) (string, error) {
	u, err := url.Parse(baseURL)
	if err != nil {
		return "", err
	}

	endpoint = strings.TrimPrefix(endpoint, "/")
	u.Path = fmt.Sprintf("%s/%s", strings.TrimSuffix(u.Path, "/"), endpoint)
	return u.String(), nil
}
