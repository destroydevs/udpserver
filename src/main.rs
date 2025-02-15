use chrono::Local;
use std::net::UdpSocket;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    // starting server for listen
    let socket = start_server();

    thread::sleep(Duration::from_secs(1)); // wait for server
    let _ = start_messages(); // start sending messages

    socket.join().unwrap(); // waiting for thread
}

fn start_messages() -> JoinHandle<()> {
    thread::spawn(|| {
        loop {
            // port randomisation (for test only)
            // can throw errors if port already use
            let port = rand::random_range(80..100);

            // create udp connection for send messages
            let udp = UdpSocket::bind(format!("127.0.0.1:{port}")).unwrap();

            // connecting to server
            udp.connect("127.0.0.1:8080").unwrap();

            // get time in millis for ping
            let time = Local::now().timestamp_millis();

            let str_to_send = format!("{},{}", rnd_sting(), time);

            // sending message
            udp.send(&str_to_send.as_bytes()).unwrap();
        }
    }) // return
}

//
// string randomisation ( for test only )
//
fn rnd_sting() -> String {
    let binding = String::from("qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM");
    let mut c_arr = binding.chars();
    let mut str = String::new();

    let count = c_arr.clone().count();

    for _c in 0..50 {
        let num = rand::random_range(0..(count-1));

        let value = c_arr.clone().nth(num);

        let _ = &str.push(value.unwrap());
    }

    str // return
}

fn start_server() -> JoinHandle<()> {
    thread::spawn(|| {
        // create listener for IP 127.0.0.1 and PORT 8080
        let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();

        // byte buffer ( if message longer than 1024 bytes it can throw error )
        let mut buf:[u8; 1024] = [0; 1024];

        // while (true)
        loop {
            // usize = bytes count
            // SocketAddr = IP address of the sender of packet
            // &socket.recv(&mut buf) getting only usize
            let r = &socket.recv_from(&mut buf).unwrap();

            if r.0 == 0 { // if message is empty or null or void etc.
                continue;
            }

            // String::from_utf8_lossy(&buf) includes the entire 1024 byte buffer
            // therefore need to trim the bytes to their real count ( &buf[..r.0] )
            let resp = String::from_utf8_lossy(&buf[..r.0]);

            if resp.matches(',').count() != 1 { // checks number of commas in message
                continue;
            }

            // getting first 2 messages
            let mut split = resp.splitn(2, ',');
            let message = split.next().unwrap();
            let s = split.next().unwrap().trim();

            // convert string to i64
            let time_millis = s.parse::<i64>().unwrap();

            let now = Local::now();

            // counting the milliseconds it took to receiving
            let time = now.timestamp_millis()-time_millis;

            let msg = format!("MAIN | {} > {} за {}ms", r.1, message.trim(), time);

            println!("{}", msg); // writing message to console

            // you can add thread::sleep() for delays
            // but you can lose most of the bytes
        }
    }) // return
}
