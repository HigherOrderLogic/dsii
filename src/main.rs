use anyhow::{Context, Result};
use tokio::{
    select,
    signal::unix::{SignalKind, signal},
};
use zbus::{Connection, Error as ZError, proxy};

#[proxy(
    interface = "org.freedesktop.ScreenSaver",
    default_service = "org.freedesktop.ScreenSaver",
    default_path = "/org/freedesktop/ScreenSaver"
)]
trait ScreenSaver {
    fn inhibit(&self, application_name: &str, reason: &str) -> Result<u32, ZError>;

    fn un_inhibit(&self, cookie: u32) -> Result<(), ZError>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::session()
        .await
        .context("failed to create dbus session")?;
    let proxy = ScreenSaverProxy::new(&conn)
        .await
        .context("failed to create proxy")?;
    let cookie = proxy
        .inhibit(env!("CARGO_PKG_NAME"), "Invoked by user")
        .await
        .context("faied to inhibit")?;

    let mut sigterm =
        signal(SignalKind::terminate()).context("failed to install SIGTERM handler")?;
    let mut sigint = signal(SignalKind::interrupt()).context("failed to install SIGINT handler")?;

    select! {
        _ = sigterm.recv() => (),
        _ = sigint.recv() => ()
    }

    proxy
        .un_inhibit(cookie)
        .await
        .context("failed to uninhibit")?;

    Ok(())
}
