# Salient
### Simple http webserver written in rust.

## Features

- [x] - Multithreading.
- [x] - Easy set up with the config file.
- [x] - No known vulnerabilities.
- [x] - Cache code or get on the fly.
- [x] - Statistics.

## Build and install
```sh
    git clone https://github.com/GreatC0der/salient.git
    cd salient
    cargo install --path .
```

Just to check run:
```sh
    salient-http
```
It shall print a link. Click on it and you shall see the message!

## Usage
Move your website files to www folder. Run `salient-http`.
Config will be generated in default folder for configs, for example `~/.config` on linux.

## Config
- `address` - Address of web server.
- `double_dot_defence` - Restrict access out of www derictory.
- `caching` - If enabled data will be taken from the memory, if disabled data will be loaded from a file every time it's requested.
- `statistics` - Count requests.
- `statistics_output_delay` - if number of requests is devisible by this value it will be displayed.
- `thread_limit` - How many threads should be used to handle requests?