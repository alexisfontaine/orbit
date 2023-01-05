use std::cell::Cell;
use std::rc::Rc;

use gloo_events::EventListener;
use leptos::*;
use web_sys::{KeyboardEvent, PointerEvent};

use crate::model::Scene;

use super::State;
use super::frames::{Frames, FramesProps};
use super::overlay::{Overlay, OverlayProps};


#[component]
pub fn Viewer (scope: Scope, scene: Scene) -> impl IntoView {
	let state = State::new(scope, scene);

	let pointer_state = Rc::new(Cell::new(None));
	let viewport = state.viewport;

	{
		let mut state = state.clone();

		let handler = EventListener::new(&document(), "keydown", move |event| match KeyboardEvent::code(event.unchecked_ref()).as_str() {
			"ArrowDown" => state.previous_camera(),
			"ArrowLeft" => state.previous_viewport(),
			"ArrowRight" => state.next_viewport(),
			"ArrowUp" => state.next_camera(),
			_ => {}
		});

		on_cleanup(scope, || drop(handler));
	}

	provide_context(scope, state.clone());

	let start = {
		let pointer_state = pointer_state.clone();

		move |event: PointerEvent| {
			event.prevent_default();

			// let event: &PointerEvent = event.unchecked_ref();

			pointer_state.set(event.is_primary().then(|| (
				document().body().unwrap().client_width() as f32,
				state.viewports_untracked().len() as f32,
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
					viewport.update(|index| *index = (*index as isize + offset).rem_euclid(size as _) as _);
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
