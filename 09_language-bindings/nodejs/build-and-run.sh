#!/bin/bash
cd rustbuch_hello
npm install
cd ..
cd rustbuch_usage
npm link ../rustbuch_hello/
npm start