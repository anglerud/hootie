# Fake Alerta

A very simple little http server that serves up alerts that look like what
might come from Alerta. I used this to test Hootie.


## Install

This uses Pipenv to manage its dependencies. Install them and activate them
like this:

```bash
$ pipenv install
$ pipenv shell
```

Note that this requires Python 3 - it won't run with Python 2.


## Usage

Run it, it'll start a webserver on port 8080.

```bash
$ ./fake_alerta.py
```
