
flatc --rust src/fbs/task.fbs --force-empty-vectors
mv task_generated.rs src/task_generated.rs
cargo test