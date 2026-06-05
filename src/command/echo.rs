pub fn execute(command: &str) -> Option<&str> {
    let parts: Vec<&str> = command.splitn(2, ' ').collect();
    parts.get(1).copied()
}