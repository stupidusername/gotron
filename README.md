# GoTron

![GoTron](gotron.webp)


## TODO

- Review & refactor
- Refactor common print functionality
- Remove compat call from main() ?
- Add pretty print for command output
- Add option to chose between pretty print and RAW output (check that the rick and morty crate outputs the raw format)
- Fix: Server panics if api key file access fails -> must return 500 response
- Findout if the api keys path is relative to the process or the working directory (fix if needed)
- Add API key validation for proxy endpoints
- Cache proxy server results
- Support binding address and port customization through env vars
- Change base URL of response objects
- Use Redis as DB for API Keys and cache Backend?
- Add documentation to readme:
    - Build and run
    - Commands and environment variables (server address, port and cache size)
    - Proxy endpoints:
        - Explain authorization
        - /singup (POST only)
        - /api | /graphql
        - Proxy server limitations