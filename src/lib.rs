mod utils;

use lazy_static::lazy_static;
use nalgebra::Vector2;
use rand::random;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// get_width/get_height TODO

// const RADIUS: f32 = H / 2.;

const REST_DENS: f32 = 1000.; // rest density
const GAS_CONST: f32 = 2000.; // const for equation of state
const H: f32 = 16.; // kernel radius
const HSQ: f32 = H * H; // radius^2 for optimization
const MASS: f32 = 65.; // assume all particles have the same mass
const VISC: f32 = 250.; // viscosity constant
const DT: f32 = 0.0008; // integration timestep

// simulation parameters
const EPS: f32 = H; // boundary epsilon
const BOUND_DAMPING: f32 = -0.5;

//const MAX_PARTICLES: usize = 2500;
//const DAM_PARTICLES: usize = 500;
//const BLOCK_PARTICLES: usize = 250;

// re projection parameters
const WINDOW_WIDTH: usize = 533;
const WINDOW_HEIGHT: usize = 400;
const VIEW_WIDTH: f32 = 1.5 * WINDOW_WIDTH as f32; // 800
const VIEW_HEIGHT: f32 = 1.5 * WINDOW_HEIGHT as f32; // 600

lazy_static! {
    // external (gravitational) forces
    static ref G: Vector2<f32> = Vector2::new(0., 12000.*-9.8);
    // smoothing kernels defined in Müller and their gradients
    static ref POLY6: f32 = 315./(65.*PI*H.powf(9.));
    static ref SPIKY_GRAD: f32 = -45./(PI*H.powf(6.));
    static ref VISC_LAP: f32 = 45./(PI*H.powf(6.));
}

#[derive(Clone, PartialEq, Debug)]
struct Particle {
    x: Vector2<f32>,
    v: Vector2<f32>,
    f: Vector2<f32>,
    r: f32,
    p: f32,
}

#[wasm_bindgen]
pub struct State {
    particles: Vec<Particle>,
}

impl Particle {
    fn new(x: f32, y: f32) -> Particle {
        Particle {
            x: Vector2::new(x, y),
            v: Vector2::new(0., 0.),
            f: Vector2::new(0., 0.),
            r: 0.,
            p: 0.,
        }
    }
}

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        utils::set_panic_hook();
        let mut particles = vec![];
        // x iter
        for i in 10..50 {
            // 50
            // y iter
            for j in 10..35 {
                let jitter = random::<f32>();
                particles.push(Particle::new(i as f32 * H + jitter, j as f32 * H));
            }
        }
        State { particles }
    }

    pub fn xs(&self) -> *const f32 {
        let mut xs: Vec<f32> = self.particles.iter().map(|p| p.x.x).collect();
        let mut ys: Vec<f32> = self.particles.iter().map(|p| p.x.y).collect();
        xs.append(&mut ys);
        xs.as_ptr()
    }

    pub fn update(&mut self) {
        self.density_pressure();
        self.forces();
        self.integrate();
    }
}

impl State {
    fn density_pressure(&mut self) {
        let particles_clone = self.particles.clone();
        for p_i in &mut self.particles {
            p_i.r = 0.;
            for p_j in &particles_clone {
                let rij: Vector2<f32> = p_j.x - p_i.x;
                let r2 = rij.norm_squared();
                if r2 < HSQ {
                    p_i.r += MASS * (*POLY6) * (HSQ - r2).powf(3.);
                }
            }
            p_i.p = GAS_CONST * (p_i.r - REST_DENS);
        }
    }

    fn forces(&mut self) {
        let l = self.particles.len();
        for i in 0..l {
            let mut fpress = Vector2::new(0., 0.);
            let mut fvisc = Vector2::new(0., 0.);
            for j in 0..l {
                if i == j {
                    continue;
                }
                let p_i = &self.particles[i];
                let p_j = &self.particles[j];
                let rij = p_j.x - p_i.x;
                let r = rij.norm();
                if r < H {
                    // compute pressure force contribution
                    fpress += -rij.normalize() * MASS * (p_i.p + p_j.p) / (2. * p_j.r)
                        * (*SPIKY_GRAD)
                        * (H - r).powf(2.);
                    // compute viscosity force contribution
                    fvisc += VISC * MASS * (p_j.v - p_i.v) / p_j.r * (*VISC_LAP) * (H - r);
                }
            }
            let fgrav = (*G) * self.particles[i].r;
            self.particles[i].f = fpress + fvisc + fgrav;
        }
    }

    fn integrate(&mut self) {
        for p in &mut self.particles {
            // forward Euler integration
            p.v += DT * p.f / p.r;
            p.x += DT * p.v;

            // enforce boundary conditions
            if p.x[0] - EPS < 0. {
                p.v[0] *= BOUND_DAMPING;
                p.x[0] = EPS;
            }
            if p.x[0] + EPS > VIEW_WIDTH {
                p.v[0] *= BOUND_DAMPING;
                p.x[0] = VIEW_WIDTH - EPS;
            }
            if p.x[1] - EPS < 0. {
                p.v[1] *= BOUND_DAMPING;
                p.x[1] = EPS;
            }
            if p.x[1] + EPS > VIEW_HEIGHT {
                p.v[1] *= BOUND_DAMPING;
                p.x[1] = VIEW_HEIGHT - EPS;
            }
        }
    }
}
