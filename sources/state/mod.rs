use std::rc::Rc;

use leptos::{
	create_memo,
	create_rw_signal,
	provide_context,
	use_context,
	MaybeSignal,
	Memo,
	RwSignal,
	Scope,
	Signal,
	UntrackedGettableSignal,
};

use crate::model::{Camera, Scene, Viewport};


#[derive(Copy, Clone)]
#[must_use]
pub struct State {
	pub(crate) camera_aspect_ratio: Memo<f64>,

	camera: RwSignal<usize>,
	overlay_enabled: Signal<bool>,
	overlay_mounted: RwSignal<bool>,
	scene: Signal<Rc<Scene>>,
	viewport: RwSignal<usize>,

	#[cfg(feature = "canvas")]
	size: RwSignal<Option<(f64, f64)>>,
}


impl State {
	#[inline]
	fn new (scope: Scope, scene: MaybeSignal<Rc<Scene>>, overlay: MaybeSignal<bool>) -> Self {
		let camera = create_rw_signal(scope, 0);

		let scene = match scene {
			MaybeSignal::Dynamic(scene) => scene,
			MaybeSignal::Static(scene) => Signal::from(create_rw_signal(scope, scene)),
		};

		Self {
			camera,
			camera_aspect_ratio: create_memo(scope, move |_|
				scene.with(|scene| scene.cameras[camera.get()].aspect_ratio.value())),
			overlay_enabled: match overlay {
				MaybeSignal::Dynamic(overlay) => overlay,
				MaybeSignal::Static(overlay) => Signal::from(create_rw_signal(scope, overlay)),
			},
			overlay_mounted: create_rw_signal(scope, false),
			scene,
			viewport: create_rw_signal(scope, 0),

			#[cfg(feature = "canvas")]
			size: create_rw_signal(scope, None),
		}
	}

	#[inline]
	#[must_use]
	pub fn cameras_untracked (&self) -> usize {
		self.scene.with_untracked(|scene| scene.cameras.len())
	}

	#[inline]
	#[must_use]
	pub fn get_camera (&self) -> usize {
		self.camera.get()
	}

	#[inline]
	#[must_use]
	pub fn get_scene (&self) -> Rc<Scene> {
		self.scene.get()
	}

	#[inline]
	#[must_use]
	pub fn get_viewport (&self) -> usize {
		self.viewport.get()
	}

	#[inline]
	#[must_use]
	pub fn is_overlay_enabled (&self) -> bool {
		self.overlay_enabled.get()
	}

	#[inline]
	#[must_use]
	pub fn is_overlay_mounted (&self) -> bool {
		self.overlay_mounted.get()
	}

	#[inline]
	pub fn set_camera (&self, camera: usize) {
		let index = self.camera.get_untracked();

		if camera != index {
			self.camera.set(camera);
		}
	}

	#[inline]
	pub fn set_viewport (&self, viewport: usize) {
		let index = self.viewport.get_untracked();

		if viewport != index {
			self.viewport.set(viewport);
		}
	}

	#[inline]
	pub fn set_overlay_mounted (&self, mounted: bool) {
		self.overlay_mounted.set(mounted);
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
	#[must_use]
	pub fn viewports (&self) -> usize {
		self.with_viewports(<[_]>::len)
	}

	#[inline]
	#[must_use]
	pub fn viewports_untracked (&self) -> usize {
		self.scene.with_untracked(|scene| scene.cameras[self.camera.get_untracked()].viewports.len())
	}

	#[inline]
	#[must_use]
	pub fn with_camera<Type> (&self, with: impl FnOnce(&Camera) -> Type) -> Type {
		self.with_cameras(|cameras| with(&cameras[self.camera.get()]))
	}

	#[inline]
	#[must_use]
	pub fn with_cameras<Type> (&self, with: impl FnOnce(&[Camera]) -> Type) -> Type {
		self.scene.with(|scene| with(&scene.cameras))
	}

	#[inline]
	#[must_use]
	pub fn with_viewport<Type> (&self, with: impl FnOnce(&Viewport) -> Type) -> Type {
		self.with_viewports(|viewports| with(&viewports[self.viewport.get()]))
	}

	#[inline]
	#[must_use]
	pub fn with_viewports<Type> (&self, with: impl FnOnce(&[Viewport]) -> Type) -> Type {
		self.with_camera(|camera| with(&camera.viewports))
	}

	#[cfg(feature = "canvas")]
	#[inline]
	#[must_use]
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
pub fn provide_viewer_state (scope: Scope, scene: MaybeSignal<Rc<Scene>>, overlay: MaybeSignal<bool>) {
	let state = State::new(scope, scene, overlay);

	provide_context(scope, state);
}

/// # Panics
///
/// Need to be called in the context of a `State`
#[inline]
pub fn use_viewer_state (scope: Scope) -> State {
	use_context(scope).unwrap()
}
