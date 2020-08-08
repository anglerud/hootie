# Fake Alerta

A very simple little http server that serves up alerts that look like what
might come from Alerta. I used this to test Hootie.


## Install

This uses [Poetry](https://python-poetry.org/) to manage its dependencies.

```bash
$ poetry install
```

Note that this requires Python 3 - it won't run with Python 2.


## Usage

Run with poetry:

```bash
$ poetry run fake_alerta
```

It'll start a webserver on port 8080.
