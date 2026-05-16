.PHONY: install build package test

install:
	cd src/vscode && npm ci

build:
	cd src/vscode && npm run compile

package:
	cd src/vscode && ./build.sh

test: package
	cd src/vscode && code --install-extension http-file-generator-*.vsix --force

all: install build package
