fn main() {
    scosmic::generate_import().unwrap();
    slint_build::compile("ui/window.slint").unwrap();
}
