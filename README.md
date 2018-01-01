# Hootie

Terminal view of Alerta alerts.

![screenshot](./hootie.png)


## Installation

You can download it from the releases, here on github, or you can install it
with cargo:

```bash
$ cargo install hootie
```


## Usage

Hootie only takes one parameter - the url to the alerta instance:

```bash
$ hootie  --alerta-url=http://localhost:8080
```


## Fake Alerta

There is a small python script in the `fake_alerta` dir that can pretend to be
alerta while you're working on Hootie. It's very simple.
