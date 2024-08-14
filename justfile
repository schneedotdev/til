alias c := create

# create an entry
create:
  cargo run -q -- that -m "message created" -t "new_message"