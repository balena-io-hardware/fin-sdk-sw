#!/usr/bin/env node

const gi = require('node-gtk')
const sleep = require('sleep')

Fin = gi.require('Fin', '0.1')

const fin = new Fin.Client()

console.log(fin.revision)

while (true) {
    sleep.sleep(10);
}
