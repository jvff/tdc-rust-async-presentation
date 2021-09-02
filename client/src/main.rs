use presentrs::{Presentrs, Properties};

fn main() {
    let properties = Properties::default().with_locales(["en", "pt"]);

    yew::start_app_with_props::<Presentrs>(properties);
}
