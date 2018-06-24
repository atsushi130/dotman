
#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Settings {
    pub repository: String,
    pub dotfiles: Vec<Dotfile>
}


#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Dotfile {
    pub name: String,
    pub input: String,
    pub output: String
}