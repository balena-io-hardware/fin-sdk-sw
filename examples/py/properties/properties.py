#!/usr/bin/env python

import signal
import gi

gi.require_version("Fin", "0.2")

from gi.repository import Fin

fin = Fin.Client.new()

print("Revision (property):", fin.props.revision)

print("Revision (method):", fin.get_revision())

print("EEPROM (property):", fin.props.eeprom)

print("EEPROM (method):", fin.get_eeprom())

print("UID (property):", fin.props.uid)

print("UID (method):", fin.get_uid())

signal.pause()
