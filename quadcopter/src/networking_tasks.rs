use embassy_net::{
    tcp::TcpSocket,
    IpListenEndpoint,
    Runner,
    Stack,
};
use esp_wifi::wifi::{
    AccessPointConfiguration,
    AuthMethod,
    WifiController,
    WifiDevice,
    WifiEvent,
    WifiState,
};
use crate::{
    debug_println,
    println,
};
use core::str::FromStr;
use embassy_time::{Timer, Duration};
use core::sync::atomic::Ordering;
use crate::{
    ControlVals,
    CONTROLS,
    CURRENT_X,
    CURRENT_Y,
    CURRENT_Z,
    CONTROLLER_CONNECTED,
    CONTROLLER_DISCONNECTED,
};



#[embassy_executor::task]
pub async fn manage_receiver_connection(stack: Stack<'static>) {
    let mut rx_buffer = [0; 2048];
    let mut tx_buffer = [0; 2048];
    debug_println!("Receiver connection management up");
    loop {
        let mut sock = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        debug_println!("Waiting for connection...");
        if let Err(e) = sock
            .accept(IpListenEndpoint {
                addr: None,
                port: 4200,
            })
                .await {
            debug_println!("Socket connection error in control loop, continuing: {:?}", e);
            continue
        }
        CONTROLLER_CONNECTED.signal(());
        debug_println!("Got tcp connection");
        let mut buf = [0u8; 4];
        loop {
            match sock.read(&mut buf).await {
                Ok(0) => {
                    debug_println!("Client connection closed.");
                    CONTROLLER_DISCONNECTED.store(true, Ordering::Relaxed);
                    break
                }
                Ok(len) => {
                    if len != 4 {
                        debug_println!("Got bad packet length: {}", len);
                        continue
                    }
                    debug_println!("Got packet: {:?}", &buf[0..len]);
                    let vals = ControlVals {
                        tilt_x: buf[0] as i8,
                        tilt_y: buf[1] as i8,
                        rot_z: buf[2] as i8,
                        collective: buf[3],
                    };
                    CONTROLS.update(vals);
                    debug_println!("Updated controls: {:?}", vals);
                    let x = CURRENT_X.load(Ordering::Relaxed);
                    let y = CURRENT_Y.load(Ordering::Relaxed);
                    let z = CURRENT_Z.load(Ordering::Relaxed);
                    let mut buf = [0u8; 12];
                    buf[0..4].copy_from_slice(&x.to_be_bytes());
                    buf[4..8].copy_from_slice(&y.to_be_bytes());
                    buf[8..12].copy_from_slice(&z.to_be_bytes());
                    let _ = sock.write(&mut buf).await;
                }
                Err(e) => {
                    debug_println!("Read error in control loop: {:?}", e);
                    CONTROLLER_DISCONNECTED.store(true, Ordering::Relaxed);
                    break
                }
            }
        }
    }
}

#[embassy_executor::task]
pub async fn run_dhcp(stack: Stack<'static>, gw_ip_addr: &'static str) {
    use core::net::{Ipv4Addr, SocketAddrV4};

    use edge_dhcp::{
        io::{self, DEFAULT_SERVER_PORT},
        server::{Server, ServerOptions},
    };
    use edge_nal::UdpBind;
    use edge_nal_embassy::{Udp, UdpBuffers};

    let ip = Ipv4Addr::from_str(gw_ip_addr).expect("dhcp task failed to parse gw ip");

    let mut buf = [0u8; 1500];

    let mut gw_buf = [Ipv4Addr::UNSPECIFIED];

    let buffers = UdpBuffers::<3, 1024, 1024, 10>::new();
    let unbound_socket = Udp::new(stack, &buffers);
    let mut bound_socket = unbound_socket
        .bind(core::net::SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::UNSPECIFIED,
            DEFAULT_SERVER_PORT,
        )))
        .await
        .unwrap();

    loop {
        _ = io::server::run(
            &mut Server::<_, 256>::new_with_et(ip),
            &ServerOptions::new(ip, Some(&mut gw_buf)),
            &mut bound_socket,
            &mut buf,
        )
        .await
        .inspect_err(|e| debug_println!("DHCP server error: {e:?}"));
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task(pool_size=10)]
pub async fn manage_ap_connection(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::ApStarted => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::ApStop).await;
                Timer::after(Duration::from_millis(100)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let wifi_config = esp_wifi::wifi::Configuration::AccessPoint(
                AccessPointConfiguration {
                    ssid: "esp_quad_wifi".try_into().unwrap(),
                    ssid_hidden: false,
                    auth_method: AuthMethod::WPA2Personal,
                    password: "rofl1337".try_into().unwrap(),
                    ..Default::default()
                }
            );
            controller.set_configuration(&wifi_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");
        }
    }
}

#[embassy_executor::task]
pub async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
