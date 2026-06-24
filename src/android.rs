use android_activity::AndroidApp;
use spdlog::error;
use winio::prelude::*;

use crate::model::MainModel;
use crate::startup::Startup;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    let init = Startup::default();

    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let app = App::builder()
        .android_app(app)
        .build()
        .expect("cannot create app");
    app.spawn(|| async {
        if let Err(e) = MainModel::run_until_event(init).await {
            error!("App error: {e:?}");
        }
    })
}

#[link(name = "c++_shared")]
unsafe extern "C" {}
