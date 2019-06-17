#!/usr/bin/env python

import gi

gi.require_version("Fin", "0.1")

from gi.repository import Fin

fin = Fin.Client.new()

print("Revision (props):", fin.props.revision)

print("Revision (method):", fin.get_revision())

