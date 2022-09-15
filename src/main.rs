mod geom;
mod vec;

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use ordered_float::NotNan;
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
    fg: Vec<Vec3>,
    #[structopt(long, default_value = "0,0,0")]
    bg: Vec3,
    #[structopt(long, default_value = "Unnamed")]
    name: String,
}

const LIGHT_DIRECTION: Vec3 = Vec3::new(-4.0 / 9.0, 8.0 / 9.0, 1.0 / 9.0);

fn compute_result(ray: Ray, scene: &Scene, opt: &Opt, bounces: usize) -> Outcome {
    // Find the closest intersection (if any)
    let maybe_intersection = scene
        .spheres
        .iter()
        .enumerate()
        .filter_map(|(i, sphere)| {
            ray.intersect_sphere(sphere)
                .map(|intersection| (i, intersection))
        })
        .min_by_key(|tuple| {
            NotNan::new(tuple.1.distance).expect("Intersection distance to be well defined")
        });

    if let Some((index, intersection)) = maybe_intersection {
        // Compute a value from 0..[number of foreground colours - 1] that
        // we can use as a position along the gradient.
        let gradient = (index * (opt.fg.len() - 1)) as f32 / scene.spheres.len() as f32;

        // Since our position is not a whole number, find the foreground colour to
        // the left of our position.
        let fg1 = opt.fg[gradient.floor() as usize];
        // And the foreground colour to our right.
        let fg2 = opt.fg[gradient.ceil() as usize];

        // And compute a value from 0..1 indicating where we are between those two colours.
        let f = gradient.fract();

        // Also apply our lighting from the previous step
        let lightness = -LIGHT_DIRECTION.dot(&intersection.normal);

        let diffuse_color = (1.0 - f) * fg1 + f * fg2;

        let combined_color = if bounces > 0 {
            // Materials tend to be more reflective as the angle of incidence increases
            let reflectivity = (1.0 - intersection.normal.dot(&ray.direction).powi(2)) * 0.7;
            let reflected_direction = ray.direction.reflection(&intersection.normal);
            // Advance the ray a small amount to avoid hitting the same sphere
            const EPSILON: f32 = 1e-6;
            let reflected_color = compute_result(
                Ray {
                    origin: intersection.position + EPSILON * reflected_direction,
                    direction: reflected_direction,
                },
                scene,
                opt,
                bounces - 1,
            )
            .color
            .expect("Color to be returned");

            (1.0 - reflectivity) * diffuse_color + reflectivity * reflected_color
        } else {
            diffuse_color
        };

        Outcome {
            hit: true,
            color: Some(combined_color + Vec3::new(lightness, lightness, lightness)),
        }
    } else {
        Outcome {
            hit: false,
            color: Some(opt.bg),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    // Connect to the server
    let mut connection = Connection::new(TcpStream::connect(opt.addr)?)?;

    // Tell the server who we are
    connection.request(Request::SetName(opt.name.clone()))?;

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
            .map(|ray| compute_result(ray, &scene, &opt, 1))
            .collect();

        // Submit the results
        connection.request(Request::SubmitResults(results))?;
    }
}
