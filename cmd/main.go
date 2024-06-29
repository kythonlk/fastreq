package main

import (
	"flag"
	"fmt"
	"github.com/kythonlk/fastreq/helper"
	"github.com/kythonlk/fastreq/types"
	"io"
	"log"
	"os"
	"strings"
)

func main() {

	initFlag := flag.Bool("init", false, "Initialize new config file")
	configFile := flag.String("F", "", "Configuration file")
	url := flag.String("X", "", "URL to send request to")
	method := flag.String("M", "GET", "HTTP method")
	headers := flag.String("H", "", "Headers (format: key=value)")
	body := flag.String("B", "", "Request body")

	flag.Parse()

	if *initFlag {
		err := helper.InitializeConfig("config.json")
		if err != nil {
			log.Fatalf("Error initializing config file: %v", err)
		}
		return
	}

	if *configFile == "" && *url == "" {
		fmt.Println("Please specify a configuration file (-F) or URL (-X)")
		flag.Usage()
		os.Exit(1)
	}

	var config *types.Config
	var err error

	if *configFile != "" {
		config, err = helper.ReadConfig(*configFile)
		if err != nil {
			log.Fatalf("Error reading config file: %v", err)
		}
	} else {
		headersMap := make(map[string]string)
		if *headers != "" {
			for _, header := range strings.Split(*headers, ",") {
				parts := strings.SplitN(header, "=", 2)
				if len(parts) == 2 {
					headersMap[parts[0]] = parts[1]
				}
			}
		}
		config = &types.Config{
			URL:     *url,
			Method:  *method,
			Headers: headersMap,
			Body:    *body,
		}
	}

	response, err := helper.SendRequest(config)
	if err != nil {
		log.Fatalf("Error sending request: %v", err)
	}
	defer response.Body.Close()

	bodyResponse, err := io.ReadAll(response.Body)
	if err != nil {
		log.Fatalf("Error reading response body: %v", err)
	}

	fmt.Printf("Response Status: %s\n", response.Status)
	fmt.Printf("Response Headers:\n%s\n", helper.FormatHeaders(response.Header))
	fmt.Printf("Response Body: %s\n", bodyResponse)
}
