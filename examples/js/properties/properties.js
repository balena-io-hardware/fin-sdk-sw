#!/usr/bin/env node

const gi = require('node-gtk')
const sleep = require('sleep')

Fin = gi.require('Fin', '0.2')

const fin = new Fin.Client()

console.log("Revision:", fin.revision)

console.log("EEPROM:", fin.eeprom)

console.log("UID:", fin.uid)

while (true) {
    sleep.sleep(10);
}
