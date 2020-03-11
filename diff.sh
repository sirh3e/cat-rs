#!/bin/bash
diff <(./target/debug/cat-rs ./src/main.rs ./src/main.rs) <(cat ./src/main.rs ./src/main.rs)
