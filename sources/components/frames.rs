use leptos::*;

use crate::state::use_state;

use super::frame::{Frame, FrameProps};


#[component]
pub fn Frames (scope: Scope) -> impl IntoView {
	let container = NodeRef::<HtmlElement<Div>>::new(scope);
	let state = use_state(scope);

	// Sets the aspect-ratio of the container to match the active camera
	create_effect(scope, move |_| {
		let container = container()?;

		state.with_camera(|camera| container.style().set_property("aspect-ratio", &camera.aspect_ratio)).ok()
	});

	// Updates the size of the container
	#[cfg(feature = "canvas")]
	{
		use wasm_bindgen::closure::Closure;
		use web_sys::{ResizeObserver, ResizeObserverEntry, ResizeObserverSize};


		let update = Closure::<dyn Fn(Vec<ResizeObserverEntry>)>::new(move |entries: Vec<ResizeObserverEntry>| {
			let size: ResizeObserverSize = entries[0].content_box_size().get(0).unchecked_into();

			state.set_size(Some((size.inline_size(), size.block_size())));
		});

		let observer = store_value(scope, ResizeObserver::new(update.as_ref().unchecked_ref()).ok());

		create_effect(scope, move |_| {
			observer.get()?.observe(&&*container()?);
			Some(())
		});

		on_cleanup(scope, move || {
			if let Some(observer) = observer.get() {
				observer.disconnect();
			}

			drop(update);
		});
	}

	// Restores the current viewport on page reload
	// FIXME: Disable `scrollRestoration` and save the current state in the `SessionStorage`.
	create_effect(scope, move |_| {
		let container = container()?;

		state.set_viewport((container.scroll_top() as f64 / container.client_height() as f64).round() as _);
		Some(())
	});

	// Displays the active viewport
	create_effect(scope, move |_| {
		let container = container()?;

		container.scroll_with_x_and_y(0., container.scroll_height() as f64 / state.viewports() as f64 * state.get_viewport() as f64);
		Some(())
	});

	view!(scope,
		<div _ref=container class="frames">
			// Purposely iterates over indexes to re-use existing nodes
			<For
				each=move || 0..state.viewports()
				key=|&index| index
				view={move |index| view!(scope, <Frame viewport=index />)}
			/>
		</div>
	)
}
