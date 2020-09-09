use argh::FromArgs;

#[derive(FromArgs)]
/// Отправить число на суммирование
pub struct Config {
    #[argh(option, short='h')]
    /// адрес удаленного сумматора
    pub host: String,

    #[argh(option, short='p')]
    /// порт удаленного сумматора
    pub port: u16,

    #[argh(option, short='n')]
    /// число, отправляемое на суммирование
    pub number: i32,

    #[argh(option, short='k')]
    /// ключ доступа к сумме на сервере
    pub key: Option<String>,
}
