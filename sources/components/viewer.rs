use gloo_events::EventListener;
use leptos::*;
use web_sys::KeyboardEvent;

use crate::model::Scene;

use super::State;
use super::frames::{Frames, FramesProps};
use super::overlay::{Overlay, OverlayProps};


#[component]
pub fn Viewer (scope: Scope, scene: Scene) -> impl IntoView {
	let state = State::new(scope, scene);

	{
		let mut state = state.clone();

		let keydown = EventListener::new(&document(), "keydown", move |event| match KeyboardEvent::code(event.unchecked_ref()).as_str() {
			"ArrowDown" => state.previous_camera(),
			"ArrowLeft" => state.previous_viewport(),
			"ArrowRight" => state.next_viewport(),
			"ArrowUp" => state.next_camera(),
			_ => {}
		});

		on_cleanup(scope, || drop(keydown));
	}

	provide_context(scope, state);

	view!(scope,
		<main class="viewer">
			<Frames />

			<Overlay />
		</main>
	)
}
