![GoTron](gotron.webp)

# GoTron

A CLI application for [The Rick and Morty API](https://rickandmortyapi.com/).

```bash
$ ./gotron character 1 --output pretty
```
```bash
Rick Sanchez
------------
id: 1
name: Rick Sanchez
status: Alive
species: Human
character_type:
origin: Earth (C-137)
location: Citadel of Ricks
gender: Male
created: 2017-11-04T18:48:46.250Z
```

## Features
- List all characters, locations and episodes.
- Get a single character, location or episode by its ID.
- Choose between getting the results in JSON or a human readable format.
- Launch a proxy server.

---

## Build

```bash
$ cargo build --release
```

## Run the application

```bash
$ ./target/release/gotron <COMMAND>
```

---

## Usage

```
Usage: gotron <COMMAND>

Commands:
  character   Get a single character by its ID
  characters  Get all characters
  location    Get a location character by its ID
  locations   Get all locations
  episode     Get a single episode by its ID
  episodes    Get all episodes
  gogotron    Start proxy server
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### Print the help of a given subcommand

```bash
$ ./gotron help character
```
```bash
Get a single character by its ID

Usage: gotron character --output <OUTPUT> <ID>

Arguments:
  <ID>

Options:
  -o, --output <OUTPUT>  [possible values: json, pretty]
  -h, --help             Print help information
```

### Choose the output format

- JSON

    ```bash
    $ ./gotron character 18 --output json
    ```
    ```bash
    {"id":18,"name":"Antenna Morty","status":"Alive","species":"Human","type":"Human with antennae","origin":{"name":"unknown","url":""},"location":{"name":"Citadel of Ricks","url":"https://rickandmortyapi.com/api/location/3"},"gender":"Male","image":"https://rickandmortyapi.com/api/character/avatar/18.jpeg","episode":["https://rickandmortyapi.com/api/episode/10","https://rickandmortyapi.com/api/episode/28"],"url":"https://rickandmortyapi.com/api/character/18","created":"2017-11-04T22:25:29.008Z"}
    ```

- Pretty

    ```bash
    $ ./gotron character 18 --output pretty
    ```
    ```bash
    Antenna Morty
    -------------
    id: 18
    name: Antenna Morty
    status: Alive
    species: Human
    character_type: Human with antennae
    origin: unknown
    location: Citadel of Ricks
    gender: Male
    created: 2017-11-04T22:25:29.008Z
    ```

---

## Proxy server

```bash
$ ./gotron gogotron
```
```bash
Starting proxy server on 127.0.0.1:8080
```

The `gogotron` subcommands starts a proxy server for The Rick And Morty API. The only difference between the proxy and the original service is that the proxy only accepts aunthenticated requests.

An API key can be obteined issuing a `POST` request to the endpoint `/signup`. Requests to any other endpoint must be sent this key in the authorization header (`Authorization: <api-key>`).

The proxy server will forward any request made to `/api/*` and `/graphql/*`.

### Examples

- Create an API key

    ```bash
    $ curl -X POST http://localhost:8080/signup
    ```
    ```bash
    {"api_key":"tZLfM2TGbYdwcYUs"}
    ```

- Use the ReST API

    ```bash
    $ curl --header "Authorization: tZLfM2TGbYdwcYUs" http://localhost:8080/api/character/25
    ```
    ```bash
    {"id":25,"name":"Armothy","status":"Dead","species":"unknown","type":"Self-aware arm","gender":"Male","origin":{"name":"Post-Apocalyptic Earth","url":"https://rickandmortyapi.com/api/location/8"},"location":{"name":"Post-Apocalyptic Earth","url":"https://rickandmortyapi.com/api/location/8"},"image":"https://rickandmortyapi.com/api/character/avatar/25.jpeg","episode":["https://rickandmortyapi.com/api/episode/23"],"url":"https://rickandmortyapi.com/api/character/25","created":"2017-11-05T08:54:29.343Z"}
    ```

- Use the GraphQL API

    ```bash
    $ curl -X POST --header "Authorization: tZLfM2TGbYdwcYUs" \
    --header "Content-Type: application/json" \
    --data '{"query":"query{characters(page:2,filter:{name:\"rick\"}){info{count}results{name}}}"}' \
    http://localhost:8080/graphql
    ```
    ```bash
    {"data":{"characters":{"info":{"count":107},"results":[{"name":"Mechanical Rick"},{"name":"Mega Fruit Farmer Rick"},{"name":"Morty Rick"},{"name":"Pickle Rick"},{"name":"Plumber Rick"},{"name":"Quantum Rick"},{"name":"Regional Manager Rick"},{"name":"Reverse Rick Outrage"},{"name":"Rick D. Sanchez III"},{"name":"Rick Guilt Rick"},{"name":"Rick Prime"},{"name":"Rick D-99"},{"name":"Rick D716"},{"name":"Rick D716-B"},{"name":"Rick D716-C"},{"name":"Rick Sanchez"},{"name":"Rick J-22"},{"name":"Rick K-22"},{"name":"Rick Sanchez"},{"name":"Ricktiminus Sancheziminius"}]}}}
    ```

---

## Current limitations and possible improvements

- The generated API keys are saved on a text file called `api-keys.txt` on the working directory from where `gotron` was launched. This is totally unacceptable for a production service, since it is insecure and limits the number of service instances to just one. Using an external DB would be a much better solution.
- The proxy server does not cache the results. A possible improvement could be adding support for a shared cache like Redis.
- The proxy server does not support any form of customization at the moment. Changing its settings through env vars or a config file could allow changing its binding address, DB access and cache backend.