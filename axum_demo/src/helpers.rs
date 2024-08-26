pub fn get_env(key: &str) -> String {
  std::env::var(key).expect(format!("No {} variable provided", key).as_str())
}
