fn main() {
  println!("cargo:rustc-link-arg=-s"); // Strip the final binary
  println!("cargo:rustc-link-arg=-Wl,--gc-sections"); // Remove unused sections
  println!("cargo:rustc-link-arg=-O3"); // Use -O3 optimization for C/C++ linker (if applicable)
}