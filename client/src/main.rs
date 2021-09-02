use presentrs::{Presentrs, Properties};

fn main() {
    let properties = Properties::default().with_locales(["pt"]);

    yew::start_app_with_props::<Presentrs>(properties);
}
