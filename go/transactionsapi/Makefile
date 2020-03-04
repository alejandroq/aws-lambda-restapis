FUNCTION_NAME := gotransactionsapi

.PHONY: start
start:
	go run cmd/server/main.go

.PHONY: all
all: deploy

.PHONY: test
test:
	go test ./... -coverprofile=coverage.out

.PHONY: coverage
coverage: 
	go tool cover -html=coverage.out ; go tool cover -func=coverage.out

.PHONY: lint
lint: # use golangci-lint
	golint $(shell find * -type d | egrep -v '^vendor')

.PHONY: vet
vet:
	go vet ./...

.PHONY: fmt
fmt:
	go fmt ./...

.PHONY: clean
clean:
	rm -rf ./bin

.PHONY: build
build: 
	GO111MODULE=on GOOS=linux go build -ldflags="-s -w" -o bin/lambda cmd/lambda/main.go

.PHONY: deploy
deploy: lint vet test clean build
	npx sls deploy --verbose

.PHONY: remove
remove:
	npx sls remove --verbose

.PHONY: logs
logs:
	npx sls logs -f ${FUNCTION_NAME}

.PHONY: dependencytree
dependencytree:
	godepgraph -o github.com/alejandroq/aws-lambda-restapis -s github.com/alejandroq/aws-lambda-restapis/cmd/lambda | dot -Tpng -o .github/dependencytree.png