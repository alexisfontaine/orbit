use gloo_events::EventListener;
use leptos::*;
use web_sys::{KeyboardEvent, PointerEvent};

use crate::state::use_state;

use super::frames::{Frames, FramesProps};
use super::overlay::{Overlay, OverlayProps};


#[component]
pub fn Viewer (scope: Scope) -> impl IntoView {
	let state = use_state(scope);
	let pointer_state = store_value(scope, None);

	let handler = EventListener::new(&document(), "keydown", move |event| match KeyboardEvent::code(event.unchecked_ref()).as_str() {
		"ArrowDown" => state.update_camera(-1),
		"ArrowLeft" => state.update_viewport(-1),
		"ArrowRight" => state.update_viewport(1),
		"ArrowUp" => state.update_camera(1),
		_ => {}
	});

	on_cleanup(scope, || drop(handler));

	let cancel = move |event: PointerEvent| {
		event.prevent_default();
		pointer_state.set(None);
	};

	let start = move |event: PointerEvent| {
		event.prevent_default();

		pointer_state.set(event.is_primary().then(|| (
			document().body().unwrap().client_width() as f32,
			state.viewports_untracked() as f32,
			event.client_x() as f32
		)));
	};

	let update = move |event: PointerEvent| {
		event.prevent_default();

		if let Some((width, size, origin)) = pointer_state.get() {
			let step = width / size;
			let offset = ((origin - event.client_x() as f32) / step) as isize;

			if offset != 0 {
				pointer_state.set(Some((width, size, f32::mul_add(offset as _, -step, origin))));
				state.update_viewport(offset);
			}
		}
	};

	view!(scope,
		<main
			class="viewer"
			on:pointercancel=cancel
			on:pointerdown=start
			on:pointermove=update
			on:pointerup=cancel
		>
			<Frames />

			{move || state.is_overlay_enabled().then(|| view!(scope, <Overlay />))}
		</main>
	)
}
