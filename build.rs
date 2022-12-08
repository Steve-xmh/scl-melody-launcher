fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set_icon_with_id("assets/icon.ico", "ICON");
    res.compile().unwrap();
}
