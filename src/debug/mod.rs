/*
 * for this file i will make the next shortcuts to help me debuging an formating the output
 * - confirm(x) => "[+] x"
 * - deny(x) => "[-] x"
 * - err(x) => "[!] ERROR: x"
 * - warn(x) => "[!] x"
 * - input(x) => "[?] x: "
 * - wait(x) => "[^] x..."
 * - note(x) => "[*] x"
 * - head(x) => "[=======(x)=======]"
*/

pub fn confirm(x: String) {println!("[+] {}", x)}
pub fn deny(x: String) {println!("[-] {}", x)}
pub fn err(x: String) {println!("[!] ERROR: {}", x)}
pub fn warn(x: String) {println!("[!] {}", x)}
pub fn input(x: String) {println!("[?] {}:", x)}
pub fn wait(x: String) {println!("[^] {}...", x)}
pub fn note(x: String) {println!("[*] {}", x)}
pub fn head(x: String) {println!("[=======({})=======]", x.to_uppercase())}
