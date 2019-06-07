#!/usr/bin/env node

Fin = gi.require('Fin', '0.1')

const fin = new Fin.Client()

console.log(fin.version)
