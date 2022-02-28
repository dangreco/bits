#/bin/bash

if ! command -v ack &> /dev/null
then
  echo "Please install \"ack\""
  exit
fi

ack "\"([a-zA-z_]+)\"" Cargo.toml --output "\$1" | while read line ; do
  cargo install --path ./$line
done