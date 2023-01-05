mod frame;
mod frames;
mod overlay;
mod viewer;


use std::rc::Rc;

use leptos::{create_rw_signal, RwSignal, Scope, Signal, UntrackedGettableSignal};

use crate::model::{Camera, Scene, Viewport};

pub use self::viewer::{Viewer, ViewerProps};


#[derive(Clone, Debug)]
struct State {
	pub scene: Rc<Scene>,

	camera: RwSignal<usize>,
	scope: Scope,
	#[cfg(feature = "canvas")]
	size: RwSignal<Option<(f64, f64)>>,
	viewport: RwSignal<usize>,
}


impl State {
	#[inline]
	fn new (scope: Scope, scene: Scene) -> Self {
		Self {
			camera: create_rw_signal(scope, scene.cameras.len() - 1),
			scene: Rc::new(scene),
			scope,
			#[cfg(feature = "canvas")]
			size: create_rw_signal(scope, None),
			viewport: create_rw_signal(scope, 0),
		}
	}

	#[inline]
	pub fn camera (&self) -> Signal<Camera> {
		let camera = self.camera;
		let scene = self.scene.clone();

		Signal::derive(self.scope, move || scene.cameras[camera.get()].clone())
	}

	#[inline]
	fn camera_untracked (&self) -> &Camera {
		&self.cameras()[self.camera.get_untracked()]
	}

	#[inline]
	pub fn cameras (&self) -> &[Camera] {
		&self.scene.cameras
	}

	pub fn next_camera (&mut self) {
		self.camera.update(|camera| {
			if *camera + 1 < self.cameras().len() {
				*camera += 1
			} else {
				*camera = 0;
			}
		});
	}

	pub fn next_viewport (&mut self) {
		self.viewport.update(|viewport| {
			if *viewport + 1 < self.viewports_untracked().len() {
				*viewport += 1
			} else {
				*viewport = 0;
			}
		});
	}

	pub fn previous_camera (&mut self) {
		self.camera.update(|camera| {
			if *camera > 0 {
				*camera -= 1;
			} else {
				*camera = self.cameras().len() - 1;
			}
		});
	}

	pub fn previous_viewport (&mut self) {
		self.viewport.update(|viewport| {
			if *viewport > 0 {
				*viewport -= 1;
			} else {
				*viewport = self.viewports_untracked().len() - 1;
			}
		});
	}

	#[inline]
	pub fn viewport (&self) -> Signal<Viewport> {
		let camera = self.camera();
		let viewport = self.viewport;

		Signal::derive(self.scope, move || camera.with(|camera| camera.viewports[viewport.get()].clone()))
	}

	#[inline]
	pub fn viewports (&self) -> Signal<usize> {
		let camera = self.camera();

		Signal::derive(self.scope, move || camera.with(|camera| camera.viewports.len()))
	}

	#[inline]
	pub fn viewports_untracked (&self) -> &[Viewport] {
		&self.camera_untracked().viewports
	}
}
