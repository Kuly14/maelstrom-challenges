cargo build

# ~/maelstrom/maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10

~/maelstrom/maelstrom/maelstrom test -w broadcast --bin target/debug/echo --node-count 3 --time-limit 20 --rate 10
