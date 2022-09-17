mod geom;
mod vec;

use std::{
    io::{Read, Write},
    net::TcpStream,
};

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use serde::{Deserialize, Serialize};

use geom::{Ray, Sphere};
use vec::Vec3;

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    ReserveRays,
    SubmitResults(Vec<Outcome>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub hit: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    ReserveRays(Vec<Ray>, Scene),
    SubmitResults,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
}

struct Connection {
    stream: TcpStream,
}

impl Connection {
    fn new(mut stream: TcpStream) -> anyhow::Result<Self> {
        stream.set_nodelay(true)?;
        // Indicate to the server what version of the protocol we are speaking
        stream.write_u32::<BE>(0)?;
        Ok(Self { stream })
    }

    fn request(&mut self, request: Request) -> anyhow::Result<Response> {
        // Encode request
        let request_data = serde_json::to_vec(&request)?;
        self.stream.write_u32::<BE>(request_data.len() as u32)?;
        self.stream.write_all(&request_data)?;

        // Decode response
        let response_size = self.stream.read_u32::<BE>()? as usize;
        let mut response_data = vec![0; response_size];
        self.stream.read_exact(&mut response_data)?;
        let response = serde_json::from_slice(&response_data)?;
        Ok(response)
    }
}

fn main() -> anyhow::Result<()> {
    // Connect to the server
    let mut connection = Connection::new(TcpStream::connect("127.0.0.1:1234")?)?;

    // Pull some rays and a scene from the server
    let (rays, scene) =
        if let Response::ReserveRays(rays, scene) = connection.request(Request::ReserveRays)? {
            (rays, scene)
        } else {
            panic!("Expected to receive rays");
        };

    // Compute whether each ray intersects the scene
    let results: Vec<_> = rays
        .into_iter()
        .map(|ray| Outcome {
            hit: scene
                .spheres
                .iter()
                .any(|sphere| ray.intersects_sphere(sphere)),
        })
        .collect();

    // Submit the results
    connection.request(Request::SubmitResults(results))?;

    Ok(())
}
