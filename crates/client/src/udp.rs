fn main() -> io::Result<()> {
    // Create a UDP socket bound to an arbitrary port
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Set the target server address
    let server_addr = "aos://870840475:32887".get_socket_address();
    println!("Connecting to {}", server_addr);

    // Send a message to the server
    // let message = b"Hello, UDP server!";
    // socket.send_to(message, server_addr)?;

    // Buffer to hold the received response
    let mut buf = [0u8; 1024];

    loop {
        // Receive a response from the server
        let (amt, src) = socket.recv_from(&mut buf)?;

        // Print the received response
        println!("Received {} bytes from {}: {:?}", amt, src, &buf[..amt]);

        // Clear the buffer to avoid mixing old and new data
        for byte in &mut buf {
            *byte = 0;
        }
    }

    // Ok(())
}
