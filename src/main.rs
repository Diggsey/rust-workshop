mod geom;
mod vec;

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use snap::raw::{Decoder, Encoder};
use structopt::StructOpt;

use geom::{Ray, Sphere};
use vec::Vec3;

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    ReserveRays,
    SubmitResults(Vec<Outcome>),
    SetName(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub hit: bool,
    pub color: Option<Vec3>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    ReserveRays(Vec<Ray>, Scene),
    SubmitResults,
    SetName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Scene {
    pub frame: u64,
    pub spheres: Vec<Sphere>,
}

struct Connection {
    stream: TcpStream,
}

impl Connection {
    fn new(mut stream: TcpStream) -> anyhow::Result<Self> {
        stream.set_nodelay(true)?;
        // Indicate to the server what version of the protocol we are speaking
        stream.write_u32::<BE>(2)?;
        Ok(Self { stream })
    }

    fn request(&mut self, request: Request) -> anyhow::Result<Response> {
        // Encode request
        let request_data = Encoder::new().compress_vec(&postcard::to_allocvec(&request)?)?;
        self.stream.write_u32::<BE>(request_data.len() as u32)?;
        self.stream.write_all(&request_data)?;

        // Decode response
        let response_size = self.stream.read_u32::<BE>()? as usize;
        let mut response_data = vec![0; response_size];
        self.stream.read_exact(&mut response_data)?;
        let response = postcard::from_bytes(&Decoder::new().decompress_vec(&response_data)?)?;
        Ok(response)
    }
}

#[derive(StructOpt)]
struct Opt {
    addr: SocketAddr,
    #[structopt(long, default_value = "1,1,1")]
    fg: Vec3,
    #[structopt(long, default_value = "0,0,0")]
    bg: Vec3,
    #[structopt(long, default_value = "Unnamed")]
    name: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    // Connect to the server
    let mut connection = Connection::new(TcpStream::connect(opt.addr)?)?;

    // Tell the server who we are
    connection.request(Request::SetName(opt.name))?;

    loop {
        // Pull some rays and a scene from the server
        let (rays, scene) =
            if let Response::ReserveRays(rays, scene) = connection.request(Request::ReserveRays)? {
                (rays, scene)
            } else {
                panic!("Expected to receive rays");
            };

        // Compute whether each ray intersects the scene
        // Use rayon to checks the rays in parallel.
        let results: Vec<_> = rays
            .into_par_iter()
            .map(|ray| {
                let hit = scene
                    .spheres
                    .iter()
                    .any(|sphere| ray.intersects_sphere(sphere));
                Outcome {
                    hit,
                    color: Some(if hit { opt.fg } else { opt.bg }),
                }
            })
            .collect();

        // Submit the results
        connection.request(Request::SubmitResults(results))?;
    }
}
