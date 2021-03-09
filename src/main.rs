extern crate pt;
use pt::*;
fn main() {
	println!("{}",rand::random::<f64>());
	let s=texture::Texture{};

	// let scene = scene::Scene{}
	// let material = DiffuseMaterial(White)
	// let plane = NewPlane(V(0, 0, 0), V(0, 0, 1), material)
	// scene.Add(plane)
	// let sphere = NewSphere(V(0, 0, 1), 1, material)
	// scene.Add(sphere)
	// let light = NewSphere(V(0, 0, 5), 1, LightMaterial(White, 8))
	// scene.Add(light)
	// let camera = LookAt(V(3, 3, 3), V(0, 0, 0.5), V(0, 0, 1), 50)
	// let sampler = NewSampler(4, 4)
	// let renderer = NewRenderer(&scene, &camera, sampler, 960, 540)
	// renderer.AdaptiveSamples = 128
	// renderer.IterativeRender("out%03d.png", 1000)
}
