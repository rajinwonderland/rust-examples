pub fn is_pling(n: u32) -> String {
  if n % 3 == 0 {
    return String::from("Pling");
  }
  return String::from("");
}

fn is_plang(n: u32) -> String {
  if n % 5 == 0 {
    return String::from("Plang");
  }
  return String::from("");
}

fn is_plong(n: u32) -> String {
  if n % 7 == 0 {
    return String::from("Plong");
  }
  return String::from("");
}

pub fn raindrops(n: u32) -> String {
  let mut sound = String::from("");
  let pling = is_pling(n);
  let plang = is_plang(n);
  let plong = is_plong(n);
  sound.push_str(&pling);
  sound.push_str(&plang);
  sound.push_str(&plong);
  if sound == "" {
    return n.to_string();
  }
  return sound;
}
