SHELL := /bin/sh

BINARY_NAME=my_scrobbler
BUILD_DIR=build

.PHONY: all run build clean

all: clean build

run:
	go run ./cmd/scrobbler

build:
	@mkdir -p $(BUILD_DIR)
	go build -o $(BUILD_DIR)/$(BINARY_NAME) ./cmd/scrobbler

clean:
	rm -rf $(BUILD_DIR)