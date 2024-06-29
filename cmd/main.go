package main

import (
	"flag"
	"fmt"
	"github.com/charmbracelet/lipgloss"
	"github.com/kythonlk/fastreq/helper"
	"github.com/kythonlk/fastreq/types"
	"io"
	"log"
	"os"
	"strings"
)

var title = lipgloss.NewStyle().
	BorderBottom(true).
	Bold(true).
	Foreground(lipgloss.Color("#7D56F4")).
	PaddingTop(1)

var bodyst = lipgloss.NewStyle().
	Foreground(lipgloss.Color("#04B575")).
	PaddingTop(1)

func main() {
	initFlag := flag.Bool("init", false, "Initialize new config file")
	configFile := flag.String("F", "", "Configuration file")
	urlFlag := flag.String("X", "", "URL to send request to")
	method := flag.String("M", "GET", "HTTP method")
	headers := flag.String("H", "", "Headers (format: key=value)")
	body := flag.String("B", "", "Request body")
	endpoint := flag.String("E", "", "Endpoint with arguments")

	flag.Parse()

	if *initFlag {
		err := helper.InitializeConfig("config.json")
		if err != nil {
			log.Fatalf("Error initializing config file: %v", err)
		}
		return
	}

	if *configFile == "" && *urlFlag == "" {
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

		baseURL := ""
		if config.BaseURL != "" {
			baseURL = config.BaseURL
		}

		if *endpoint != "" {
			fullURL, err := helper.ConstructFullURL(baseURL, *endpoint)
			if err != nil {
				log.Fatalf("Error constructing full URL: %v", err)
			}
			config = &types.Config{
				URL:     fullURL,
				Method:  *method,
				Headers: headersMap,
				Body:    *body,
			}
		} else if *urlFlag != "" {
			config = &types.Config{
				URL:     *urlFlag,
				Method:  *method,
				Headers: headersMap,
				Body:    *body,
			}
		} else {
			log.Fatalf("Endpoint (-E) must be specified when URL (-X) is not provided")
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
