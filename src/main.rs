use Webserver::frame::frame_struct::Frame;
use std::any::type_name;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::{io, thread};
use std::thread::sleep;
use std::time::Duration;
use Webserver::execute::Execute;
use Webserver::frame::message_types::MessageType;
use Webserver::kernel::kernel::Kernel;
use Webserver::kernel::kernel_functions::KernelFunctions;
use Webserver::narray::Narray;


fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                /* connection failed */
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    //stream.set_nonblocking(true);
    let (tx, rx) = channel();

    thread_read(stream.try_clone().unwrap(), tx);
    thread_write(stream, rx);

}

fn thread_read(mut stream: TcpStream, tx : Sender<Vec<u8>>) {
    thread::spawn(move || {
        loop {
            //println!("Still reading");
            let mut buf = vec![0; 1024];
            match stream.read(&mut buf) {
                Ok(len) => {
                    if len > 0 {
                        buf.truncate(len);
                        println!("Received data, {:?}", buf.as_slice());
                        tx.send(buf).unwrap();
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    println!("{}", e);
                }
                Err(e) => panic!("encountered IO error: {}", e),
            };
        }
    });
}

fn thread_write(mut stream : TcpStream, rx : Receiver<Vec<u8>>) {
    thread::spawn(move || {
        loop {
            for rcv in rx.recv() {
                let response = deserialize_and_create_response(rcv);
                let res = response.as_slice();
                sleep(Duration::from_secs(3));
                stream.write_all(&response).unwrap();
                stream.flush().unwrap();
                println!("Send response: {:?}", res);
            }
        }
    });
}

fn deserialize_and_create_response(data_u8: Vec<u8>) -> Vec<u8> {
    let mut response_u8: Vec<u8> = Vec::new();

    let mut res_narray: bincode::Result<Frame<Narray>> = bincode::deserialize(&data_u8);
    let mut res_execute: bincode::Result<Frame<Execute>> = bincode::deserialize(&data_u8);
    let mut res_kernel: bincode::Result<Frame<Kernel>> = bincode::deserialize(&data_u8);

    if res_narray.is_ok() {
        let mut response = res_narray.unwrap();
        if response.m_type == MessageType::Narray {
            response.data = Narray { data: Vec::from("Response from server") };
            return bincode::serialize(&response).unwrap();
        }
    }

    if res_execute.is_ok() {
        let mut response = res_execute.unwrap();
        if response.m_type == MessageType::ExecuteFn {
            response.data = Execute { function: KernelFunctions::FnAdd };
            return bincode::serialize(&response).unwrap();
        }
    }

    if res_kernel.is_ok() {
        let mut response = res_kernel.unwrap();
        if response.m_type == MessageType::Kernel {
            response.data = Kernel { operation_type: String::from("Response from server") };
            return bincode::serialize(&response).unwrap();
        }
    }

    eprintln!("Could not deserialze frame");

    response_u8
}

fn create_response_to_request(mut data_u8: Vec<u8>, len: usize) -> Vec<u8> {
    data_u8.truncate(len);

    let result_data: Frame<Narray> = bincode::deserialize(&data_u8).unwrap();
    println!("id: {}, data: {}", result_data.id, String::from_utf8_lossy(result_data.data.data.as_slice()));
    let response: Frame<Narray> = Frame::new(
        result_data.id,
        result_data.m_type,
        Narray { data: Vec::from("Response from server") },
    );
    return bincode::serialize(&response).unwrap();


    //let result_kernel : Frame<Kernel> = bincode::deserialize(&buf);
    /*
    let result_execute : Frame<Execute> = bincode::deserialize(&buf).unwrap_or_else(eprintln!("Could not deserialize to kernel"));

    let mut response_u8 : Vec<u8> = vec![];
    if result_kernel.is_ok() {
        let frame = result_kernel.unwrap();
        let response: Frame<Data> = Frame::new(
            frame.id,
            frame.m_type,
            Kernel {operation_type : String::from("Server received message") });
        response_u8 = bincode::serialize(&response).unwrap();
    }
    else if result_data.is_ok() {
        let frame = result_data.unwrap();
        let response : Frame<Narray> = Frame::new(
            frame.id,
            frame.m_type,
            Narray {data : Vec::from("Server received message")});
        response_u8 = bincode::serialize(&response).unwrap();
    }
    else if result_execute.is_ok() {
        let frame = result_execute.unwrap();
        let response : Frame<Execute> = Frame::new(
            frame.id,
            frame.m_type,
            frame.data
        );
        response_u8 = bincode::serialize(&response).unwrap();
    }
    response_u8
*/
}