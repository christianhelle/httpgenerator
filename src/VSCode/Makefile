.PHONY: install build package test

install:
	cd src/VSCode && npm install

build:
	cd src/VSCode && npm run compile

package:
	cd src/VSCode && npm run package

test: package
	cd src/VSCode && code --install-extension http-file-generator-*.vsix --force

all: install build package
