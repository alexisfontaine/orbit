mod frame;
mod frames;
mod overlay;
mod viewer;


use std::rc::Rc;

use leptos::{create_rw_signal, RwSignal, Scope, UntrackedGettableSignal};

use crate::model::{Camera, Scene, Viewport};

pub use self::viewer::{Viewer, ViewerProps};


#[derive(Clone, Debug)]
struct State {
	pub scene: Rc<Scene>,

	camera: RwSignal<usize>,
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
			#[cfg(feature = "canvas")]
			size: create_rw_signal(scope, None),
			viewport: create_rw_signal(scope, 0),
		}
	}

	#[inline]
	pub fn camera (&self) -> &Camera {
		&self.cameras()[self.camera.get()]
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
			if *camera == self.cameras().len() - 1 {
				*camera = 0;
			} else {
				*camera += 1
			}
		});
	}

	pub fn next_viewport (&mut self) {
		self.viewport.update(|viewport| {
			if *viewport == self.viewports_untracked().len() - 1 {
				*viewport = 0;
			} else {
				*viewport += 1
			}
		});
	}

	pub fn previous_camera (&mut self) {
		self.camera.update(|camera| {
			if *camera == 0 {
				*camera = self.cameras().len() - 1;
			} else {
				*camera -= 1;
			}
		});
	}

	pub fn previous_viewport (&mut self) {
		self.viewport.update(|viewport| {
			if *viewport == 0 {
				*viewport = self.viewports_untracked().len() - 1;
			} else {
				*viewport -= 1;
			}
		});
	}

	#[inline]
	pub fn viewport (&self) -> &Viewport {
		&self.viewports()[self.viewport.get()]
	}

	#[inline]
	pub fn viewports (&self) -> &Vec<Viewport> {
		&self.camera().viewports
	}

	#[inline]
	pub fn viewports_untracked (&self) -> &[Viewport] {
		&self.camera_untracked().viewports
	}
}
