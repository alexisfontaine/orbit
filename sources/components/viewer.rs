use std::cell::Cell;
use std::rc::Rc;

use gloo_events::EventListener;
use leptos::*;
use web_sys::{KeyboardEvent, PointerEvent};

use crate::model::Scene;
use crate::state::State;

use super::frames::{Frames, FramesProps};
use super::overlay::{Overlay, OverlayProps};


#[component]
pub fn Viewer (scope: Scope, scene: Scene) -> impl IntoView {
	let state = State::new(scope, scene);

	provide_context(scope, state);

	let handler = EventListener::new(&document(), "keydown", move |event| match KeyboardEvent::code(event.unchecked_ref()).as_str() {
		"ArrowDown" => state.update_camera(-1),
		"ArrowLeft" => state.update_viewport(-1),
		"ArrowRight" => state.update_viewport(1),
		"ArrowUp" => state.update_camera(1),
		_ => {}
	});

	on_cleanup(scope, || drop(handler));

	let pointer_state = Rc::new(Cell::new(None));

	let start = {
		let pointer_state = pointer_state.clone();

		move |event: PointerEvent| {
			event.prevent_default();

			pointer_state.set(event.is_primary().then(|| (
				document().body().unwrap().client_width() as f32,
				state.viewports_untracked() as f32,
				event.client_x() as f32
			)));
		}
	};

	let update = {
		let pointer_state = pointer_state.clone();

		move |event: PointerEvent| {
			event.prevent_default();

			if let Some((width, size, origin)) = pointer_state.get() {
				let step = width / size;
				let offset = ((origin - event.client_x() as f32) / step) as isize;

				if offset != 0 {
					pointer_state.set(Some((width, size, origin - offset as f32 * step)));
					state.update_viewport(offset);
				}
			}
		}
	};

	let cancel = move |event: PointerEvent| {
		event.prevent_default();
		pointer_state.set(None);
	};

	view!(scope,
		<main
			class="viewer"
			on:pointercancel=cancel.clone()
			on:pointerdown=start
			on:pointermove=update
			on:pointerup=cancel
		>
			<Frames />

			<Overlay />
		</main>
	)
}
