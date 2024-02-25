#!/usr/bin/env bash

for i in {1..25}; do
  cp template.rs day$(printf %02d $i).rs
done
