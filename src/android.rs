use android_activity::AndroidApp;
use spdlog::error;
use winio::prelude::*;

use crate::model::MainModel;
use crate::timer::Timer;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    let init = Timer::default();

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
