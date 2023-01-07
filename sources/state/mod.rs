use std::rc::Rc;

use leptos::{create_rw_signal, use_context, RwSignal, Scope, UntrackedGettableSignal};

use crate::model::{Camera, Scene, Viewport};


#[derive(Copy, Clone, Debug)]
pub struct State {
	camera: RwSignal<usize>,
	scene: RwSignal<Rc<Scene>>,
	viewport: RwSignal<usize>,

	#[cfg(feature = "canvas")]
	size: RwSignal<Option<(f64, f64)>>,
}


impl State {
	#[inline]
	pub fn new (scope: Scope, scene: Scene) -> Self {
		Self {
			camera: create_rw_signal(scope, scene.cameras.len() - 1),
			scene: create_rw_signal(scope, Rc::new(scene)),
			viewport: create_rw_signal(scope, 0),

			#[cfg(feature = "canvas")]
			size: create_rw_signal(scope, None),
		}
	}

	#[inline]
	pub fn cameras_untracked (&self) -> usize {
		self.scene.with_untracked(|scene| scene.cameras.len())
	}

	#[inline]
	pub fn get_camera (&self) -> usize {
		self.camera.get()
	}

	#[inline]
	pub fn get_scene (&self) -> Rc<Scene> {
		self.scene.get()
	}

	#[inline]
	pub fn get_viewport (&self) -> usize {
		self.viewport.get()
	}

	#[inline]
	pub fn set_viewport (&self, viewport: usize) {
		let index = self.viewport.get_untracked();

		if viewport != index {
			self.viewport.set(viewport);
		}
	}

	#[inline]
	pub fn update_camera (&self, offset: isize) {
		let index = self.camera.get_untracked();
		let camera = (index as isize + offset).rem_euclid(self.cameras_untracked() as _) as _;

		if camera != index {
			self.camera.set(camera);
		}
	}

	#[inline]
	pub fn update_viewport (&self, offset: isize) {
		let index = self.viewport.get_untracked();
		let viewport = (index as isize + offset).rem_euclid(self.viewports_untracked() as _) as _;

		if viewport != index {
			self.viewport.set(viewport);
		}
	}

	#[inline]
	pub fn viewports (&self) -> usize {
		self.with_viewports(|viewports| viewports.len())
	}

	#[inline]
	pub fn viewports_untracked (&self) -> usize {
		self.scene.with_untracked(|scene| scene.cameras[self.camera.get_untracked()].viewports.len())
	}

	#[inline]
	pub fn with_camera<Type> (&self, with: impl FnOnce(&Camera) -> Type) -> Type {
		self.with_cameras(|cameras| with(&cameras[self.camera.get()]))
	}

	#[inline]
	pub fn with_cameras<Type> (&self, with: impl FnOnce(&[Camera]) -> Type) -> Type {
		self.scene.with(|scene| with(&scene.cameras))
	}

	#[inline]
	pub fn with_viewport<Type> (&self, with: impl FnOnce(&Viewport) -> Type) -> Type {
		self.with_viewports(|viewports| with(&viewports[self.viewport.get()]))
	}

	#[inline]
	pub fn with_viewports<Type> (&self, with: impl FnOnce(&[Viewport]) -> Type) -> Type {
		self.with_camera(|camera| with(&camera.viewports))
	}

	#[cfg(feature = "canvas")]
	#[inline]
	pub fn get_size (&self) -> Option<(f64, f64)> {
		self.size.get()
	}

	#[cfg(feature = "canvas")]
	#[inline]
	pub fn set_size (&self, size: Option<(f64, f64)>) {
		let value = self.size.get_untracked();

		if size != value {
			self.size.set(size);
		}
	}
}


#[inline]
pub fn use_state (scope: Scope) -> State {
	use_context(scope).unwrap()
}
