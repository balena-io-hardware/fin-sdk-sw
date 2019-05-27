#!/usr/bin/env python

import gi

gi.require_version("Fin", "0.1")

from gi.repository import Fin

fin = Fin.Client.new()

print("Version (props):", fin.props.version)

print("Version (method):", fin.get_version())

