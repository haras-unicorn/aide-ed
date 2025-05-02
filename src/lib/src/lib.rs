#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

// TODO: fix env filter

use tracing::dispatcher::set_global_default;
use tracing::Level;

pub use tracing;

pub fn init() -> anyhow::Result<()> {
  let level = Level::INFO;

  #[cfg(target_arch = "wasm32")]
  {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::Registry;

    let layer_config = tracing_wasm::WASMLayerConfigBuilder::new()
      .set_max_level(level)
      .build();
    let layer = tracing_wasm::WASMLayer::new(layer_config);
    let reg = Registry::default().with(layer);

    console_error_panic_hook::set_once();

    set_global_default(reg.into())?;
  }

  #[cfg(not(target_arch = "wasm32"))]
  {
    tracing_log::LogTracer::init()?;

    let filter = tracing_subscriber::EnvFilter::builder()
      .with_default_directive(level.into())
      .from_env_lossy();

    let builder =
      tracing_subscriber::fmt::Subscriber::builder().with_env_filter(filter);

    if !dioxus::cli_config::is_cli_enabled() {
      set_global_default(builder.finish().into())?;
    } else {
      set_global_default(builder.without_time().finish().into())?;
    }
  }

  Ok(())
}
