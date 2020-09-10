// Сервер, принимающий 2 слагаемых и хранящий в буффере сумму 

mod config;
use config::Config;

#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream, IpAddr, SocketAddr};
use std::io::{Read};

use socket2::{Socket, Domain, Type, Protocol};

// Функция, обрабатываящая каждого клиента по отдельности последовательно
fn handle_sequentially(mut stream: TcpStream, sum_buffer: &mut Vec<i32>,
                       sum: &mut bool) -> std::io::Result<()> {
    // читаем байты из потока
    let mut rd_buf = [0u8; 4];
    stream.read_exact(&mut rd_buf)?;

    let n = i32::from_be_bytes(rd_buf); // собираем из сырых байтов число

    // Если число -- второе слагаемое, то прибавляем его к первому на конце 
    // буффера. expr_status -- статус текущего выражения для отображения.
    let expr_status = if *sum {
        *(sum_buffer.last_mut().unwrap()) += n;
        ""
    } else { // иначе число - первое слагаемое и кладется в конец буффера
        sum_buffer.push(n);
        " + /*ожидание слагаемого*/"
    };

    // Печатаем результат
    print!("====\n\
           Принято -- {}.\n\
           История: {}{}", n, sum_buffer.last().unwrap(), expr_status);
    for i in (0..sum_buffer.len() - 1).rev() {
        print!(", {}", sum_buffer[i]);
    }
    println!(".");

    *sum = !*sum; // меняем флаг

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Задаем максимальное количество клиентов в очереди (SOMAXCONN в си)
    const MAX_CONN: i32 = 128;

    let conf: Config = argh::from_env(); // смотрим параметры приложения
    let ip = conf.host.parse::<IpAddr>()
        .expect("Не удалось прочитать IP-адрес");
    let sock_addr = SocketAddr::new(ip, conf.port);

    // Удобно, однако слишком просто
    /* let listener = TcpListener::bind("127.0.0.1:8080")?; */

    // Используем более гибкий системный интерфейс.
    // Создаем сокет в домене IPv4, протокол -- TCP
    let socket = Socket::new(
        Domain::ipv4(), Type::stream(), Some(Protocol::tcp())
    )?;

    // Задаём переиспользование адреса даже если он занят
    socket.set_reuse_address(true) 
        .unwrap();

    // Назначаем сокету адрес и порт
    socket.bind(&sock_addr.into())
        .expect("Не удалось сделать bind()!");

    // Делаем сокет пассивным, слушающим входящие соединения
    socket.listen(MAX_CONN)
        .expect("Не удалось сделать listen()");

    // Преобразуем сокет в TcpListener. Этот объект может делать accept()
    let listener = socket.into_tcp_listener();

    let mut add_results: Vec<i32> = Vec::new();
    let mut sum = false;

    // listener.incoming() возвращает итератор, на каждой итерации которого
    // происходит accept(). Таким образом, это аналог while(true). Итератор
    // итерируется по TcpStream. Туда можно писать, оттуда же можно читать,
    // по аналогии с send/recv.
    for stream in listener.incoming() {
        handle_sequentially(stream?, &mut add_results, &mut sum)?;
    }

    Ok(())
}
