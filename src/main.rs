use std::{process, path};
use path::Path;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate network_manager;
use network_manager::{manager, wifi, device, connection};

extern crate iron;
use iron::{Iron, Request, Response, IronResult, status, typemap, error, Listening};
use iron::prelude::*;

extern crate router;
use router::Router;

extern crate staticfile;
use staticfile::Static;

extern crate mount;
use mount::Mount;

extern crate serde_json;

extern crate persistent;
use persistent::{State, Read, Write};

extern crate params;
use params::Params;

struct Manager_ref<'a> {
    manager: &'a manager::NetworkManager
}

pub struct Manager<'a>;
impl<'a> typemap::Key<'a> for Manager<'a> {
    type Value = Manager_ref<'a>;
}

pub struct Device;
impl typemap::Key for Device {
    type Value = device::Device;
}

pub struct AccessPoints;
impl typemap::Key for AccessPoints {
    type Value = Vec<wifi::AccessPoint>;
}

pub struct AccessPoint;
impl typemap::Key for AccessPoint {
    type Value = wifi::AccessPoint;
}

fn main() {
    // TODO error handling

    // Define command line flags
    let matches = App::new("resin-wifi-connect")
        .version("0.1.0")
        .author("Joe Roberts <joe@resin.io>")
        .about("WiFi credentials configuration tool")
        .arg(Arg::with_name("interface")
            .short("i")
            .long("interface")
            .value_name("INTERFACE")
            .help("Hotspot interface")
            .takes_value(true))
        .arg(Arg::with_name("ssid")
            .short("s")
            .long("ssid")
            .value_name("SSID")
            .help("Hotspot SSID")
            .takes_value(true))
        .arg(Arg::with_name("password")
            .short("p")
            .long("password")
            .value_name("PASSWORD")
            .help("Hotspot password ")
            .takes_value(true))
        .arg(Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .value_name("TIMEOUT")
            .help("Hotspot timeout (seconds)")
            .takes_value(true))
        .get_matches();

    // Parse command line flags
    let interface: Option<&str> = matches.value_of("interface");
    let ssid = matches.value_of("ssid").unwrap_or("resin-hotspot");
    let password: Option<&str> = matches.value_of("password");
    let timeout = matches.value_of("timeout").map_or(600, |v| v.parse::<i32>().unwrap());

    // Get manager
    let manager = manager::new();

    // Get device
    let device = get_device(&manager, interface).unwrap();

    // Start the hotspot
    let hotspot_connection = start_hotspot(&manager, &device, ssid, password).unwrap();

    // Start the server
    // TODO: handle result
    let mut access_points;
    let mut access_point;
    // pass by value
    start_server(&manager, &device, &access_points, &access_point);

    // Wait for credentials or timeout to elapse
    //
    // Stop_server + grab back values

    // Stop hotspot
    stop_hotspot(&manager, &hotspot_connection).unwrap();

    // Add and activate credentials
    let access_point_password = "hel";
    match connection::create(&manager, &device, &access_point, &access_point_password, 10) {
        Ok(connection) => process::exit(0),
        Err(error) => {
            // delete creds
            process::exit(1);
        }
    }
}

fn get_device(manager: &manager::NetworkManager,
              interface: Option<&str>)
              -> Result<device::Device, String> {
    let mut devices = device::list(&manager).unwrap();

    let index;
    if let Some(interface) = interface {
        index = devices.iter()
            .position(|ref d| d.device_type == device::DeviceType::WiFi && d.interface == interface)
            .unwrap();
    } else {
        index = devices.iter()
            .position(|ref d| d.device_type == device::DeviceType::WiFi)
            .unwrap();
    }

    Ok(devices[index].clone())
}

fn get_access_points(manager: &manager::NetworkManager,
                     device: &device::Device)
                     -> Result<Vec<wifi::AccessPoint>, String> {
    let mut access_points = wifi::scan(&manager, &device).unwrap();
    access_points.retain(|ap| ap.ssid != "");
    Ok(access_points)
}

fn start_hotspot(manager: &manager::NetworkManager,
                 device: &device::Device,
                 ssid: &str,
                 password: Option<&str>)
                 -> Result<connection::Connection, String> {
    connection::create_hotspot(&manager, &device, ssid, password.map(|p| p.to_owned()), 10)
}

fn stop_hotspot(manager: &manager::NetworkManager,
                connection: &connection::Connection)
                -> Result<(), String> {
    connection::disable(&manager, &mut connection.to_owned(), 10).unwrap();
    connection::delete(&manager, &connection)
}

fn start_server(manager: &manager::NetworkManager,
                device: &device::Device,
                access_points: &AccessPoints,
                access_point: &AccessPoint)
                -> () {
    let mut chain = Chain::new(ssids);
    chain.link(Read::<Manager>::both(manager));
    chain.link(Read::<Device>::both(device));
    chain.link(State::<AccessPoints>::both(access_points));
    chain.link(Write::<AccessPoint>::both(access_point));

    let mut router = Router::new();
    router.get("/", Static::new(Path::new("public")), "index");
    router.get("/ssids", chain, "ssids");
    router.post("/connect", connect, "connect");

    let mut assets = Mount::new();
    assets.mount("/", router);
    assets.mount("/css", Static::new(Path::new("public/css")));
    assets.mount("/img", Static::new(Path::new("public/img")));
    assets.mount("/js", Static::new(Path::new("public/js")));

    // TODO: return this result
    Iron::new(assets).http("localhost:3000").unwrap();
}

fn ssids(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<AccessPoints>>().unwrap();
    let mut access_points = mutex.lock().unwrap();
    let manager = req.get::<Read<Manager>>().unwrap();
    let device = req.get::<Read<Device>>().unwrap();
    *access_points = get_access_points(&manager, &device).unwrap();

    let payload =
        serde_json::to_string(&access_points.iter().map(|ap| ap.ssid).collect::<Vec<String>>())
            .unwrap();

    Ok(Response::with((status::Ok, payload)))
}

fn connect(req: &mut Request) -> IronResult<Response> {
    let access_points = req.get::<Read<AccessPoints>>().unwrap();

    println!("{:?}", req.get_ref::<Params>());
    // write access point
    Ok(Response::with(status::Ok))
}
