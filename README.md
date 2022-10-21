# GoTron

![GoTron](gotron.webp)


## TODO

- Review & refactor
- Refactor common print functionality
- Remove compat call from main() ?
- Add pretty print for command output
- Add option to chose between pretty print and RAW output (check that the rick and morty crate outputs the raw format)
- Save API Key to DB
- Output JSON after api-key generation
- Add API key validation for proxy endpoints
- Cache proxy server results
- Support binding address and port customization through env vars
- Change base URL of response objects
- Add documentation to readme:
    - Build and run
    - Commands and environment variables
    - Proxy endpoints:
        - Explain authorization
        - /singup (POST only)
        - /api | /graphql