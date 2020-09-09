use argh::FromArgs;

#[derive(FromArgs)]
/// Сервер, суммирующий пришедшие по сети числа
pub struct Config {
    #[argh(option, short='h')]
    /// адрес сервера
    pub host: String,

    #[argh(option, short='p')]
    /// порт сервера
    pub port: u16,
}
