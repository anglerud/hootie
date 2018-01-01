#!/usr/bin/env python
# coding=utf-8
"""Pretends to be an instance of Alerta."""
import random

from aiohttp import web


ALERTS = [
    {
        'name': 'PantsOnFire',
        'resource': 'pants',
        'severity': 'page',
        'time': '2017-12-30 13:00:00'
    },
    {
        'name': 'PantsSmouldering',
        'resource': 'pants',
        'severity': 'warn',
        'time': '2017-12-30 12:00:00'
    }
]


async def hello(request):
    """Respond with a random selection of alerts."""
    all_alerts = ALERTS[:]
    random.shuffle(all_alerts)  # Sadly, shuffle is done in place.
    selected_alerts = all_alerts[:random.randint(0, 2)]
    alerts = {
        'alerts': selected_alerts
    }

    return web.json_response(alerts)


def main() -> None:
    """Entrypoint."""
    app = web.Application()
    app.router.add_get('/', hello)

    web.run_app(app)


if __name__ == '__main__':
    main()
