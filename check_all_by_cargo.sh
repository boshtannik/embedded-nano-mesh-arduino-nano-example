#!/bin/bash
echo "Checking all examples for syntax correctness"
echo "=================="
echo "Checking broadcast"
cargo check --example broadcast
echo "Checking receive"
cargo check --example receive
echo "Checking empty_node"
cargo check --example empty_node
echo "Checking send"
cargo check --example send
echo "Checking send_ping_pong"
cargo check --example send_ping_pong
echo "Checking send_with_transaction"
cargo check --example send_with_transaction
