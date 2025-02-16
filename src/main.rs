mod crypt;

use chrono::Local;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::prelude::IndexedRandom;
use rand::{Rng, SeedableRng};

use crate::crypt::Crypt;

fn main() {
    // creating crypto encrypting
    let crypt = Arc::new(Mutex::new(Crypt::new()));

    // starting server for listen
    println!("Staring server...");
    let socket = start_server(Arc::clone(&crypt));

    thread::sleep(Duration::from_secs(1)); // wait for server
    println!("Server started.");

    let _ = start_messages(Arc::clone(&crypt)); // start sending messages

    socket.join().unwrap(); // waiting for thread
}

fn start_messages(crp: Arc<Mutex<Crypt>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {

            // changed UDP visibility zone for delays.
            // yes, I can just move thead::sleep to beginning of the loop,
            // but I wanted it to be at the end
            {
                // port randomisation (for test only)
                // can throw errors if port already use
                let port = rand::random_range(80..100);

                // create udp connection for send messages
                let udp = UdpSocket::bind(format!("127.0.0.1:{port}")).expect("Failed to bind to address");

                // connecting to server
                udp.connect("127.0.0.1:8080").expect("Failed to connect to server");

                // get time in millis for ping
                let time = Local::now().timestamp_millis();

                let rnd_num = rand::random_range(4..7);

                let str_to_send = format!("{},{}",rnd_phrase(rnd_num), time);

                // sending message
                let cr = crp.lock().unwrap();
                let encrypted_data = cr.encrypt(&str_to_send.as_bytes()).expect("Failed to encrypt data");
                udp.send(&encrypted_data).expect("Failed to send data");
            }

            // delay in sending
            thread::sleep(Duration::from_secs(3));
        }
    }) // return
}

//
// phrase randomisation ( for test only )
//
fn rnd_phrase(phrase_length: usize) -> String {
    let words = vec![
        "lazy", "developer", "can", "create", "anything",
        "but", "and", "dog", "cat", "red", "green", "blue",
        "html", "css", "java", "rust", "I", "love", "rust",
        "world", "word"
    ];
    let mut rng = rand::rng();
    let mut phrase = Vec::with_capacity(phrase_length);

    for _ in 0..phrase_length {
        let word = words.choose(&mut rng).unwrap();
        phrase.push(*word);
    }

    phrase.join(" ") // return
}

fn start_server(crp: Arc<Mutex<Crypt>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        // create listener for IP 127.0.0.1 and PORT 8080
        let socket = UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind to address");

        // byte buffer (if message longer than 1024 bytes it can throw error)
        let mut buf = [0; 1024];

        // while (true)
        loop {
            // usize = bytes count
            // SocketAddr = IP address of the sender of packet
            // &socket.recv(&mut buf) getting only usize
            let (size, ip) = socket.recv_from(&mut buf).expect("Failed to receive data");

            if size == 0 { // if message is empty or null or void etc.
                continue;
            }

            // String::from_utf8_lossy(&buf) includes the entire 1024 byte buffer
            // therefore need to trim the bytes to their real count ( &buf[..r.0] )
            let cr = crp.lock().unwrap();
            let decrypted_data = cr.decrypt(&buf[..size]).expect("Failed to decrypt data");
            let resp = String::from_utf8_lossy(&decrypted_data);

            if resp.matches(',').count() != 1 { // checks number of commas in message
                continue;
            }

            // getting first 2 messages
            let mut split = resp.splitn(2, ',');
            let message = split.next().unwrap();
            let time_str = split.next().unwrap().trim();

            // convert string to i64
            let time_millis = time_str.parse::<i64>().expect("Failed to parse time");
            let now = Local::now().timestamp_millis();

            // counting the milliseconds it took to receiving
            let time_taken = now - time_millis;

            let msg = format!("MAIN | {} > {} за {}ms", ip, message.trim(), time_taken);

            println!("{}", msg);

            // you can add thread::sleep() for delays
            // but you can lose most of the bytes
        }
    }) // return
}

// string randomisation ( for test only )
// (not used)
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
