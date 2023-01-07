use leptos::*;

use super::State;
use super::frame::{Frame, FrameProps};


#[component]
pub fn Frames (scope: Scope) -> impl IntoView {
	let state = use_context::<State>(scope).unwrap();

	let camera = state.camera();
	let container = NodeRef::<HtmlElement<Div>>::new(scope);
	let viewports = state.viewports();

	// Sets the aspect-ratio of the container to match the active camera
	create_effect(scope, move |_| {
		let style = container()?.style();

		camera.with(|camera| style.set_property("aspect-ratio", &camera.aspect_ratio)).ok()
	});

	// Updates the size of the container
	#[cfg(feature = "canvas")]
	{
		use std::rc::Rc;

		use wasm_bindgen::closure::Closure;
		use web_sys::{ResizeObserver, ResizeObserverEntry, ResizeObserverSize};


		let update = Closure::<dyn Fn(Vec<ResizeObserverEntry>)>::new(move |entries: Vec<ResizeObserverEntry>| {
			let size: ResizeObserverSize = entries[0].content_box_size().get(0).unchecked_into();

			state.size.set(Some((size.inline_size(), size.block_size())));
		});

		let observer = Rc::new(ResizeObserver::new(update.as_ref().unchecked_ref()).unwrap());

		{
			let observer = Rc::downgrade(&observer);

			create_effect(scope, move |_| {
				observer.upgrade()?.observe(&&*container()?);
				Some(())
			});
		}

		on_cleanup(scope, move || {
			observer.disconnect();
			drop(update);
		});
	}

	// Restores the current viewport on page reload
	// FIXME: Disable `scrollRestoration` and save the current state in the `SessionStorage`.
	create_effect(scope, move |_| {
		let container = container()?;
		let index = (container.scroll_top() as f64 / container.client_height() as f64).round() as _;

		if index != state.viewport.get_untracked() {
			state.viewport.set(index);
		}

		Some(())
	});

	// Displays the active viewport
	create_effect(scope, move |_| {
		let container = container()?;

		container.scroll_with_x_and_y(0., container.scroll_height() as f64 / viewports() as f64 * state.viewport.get() as f64);
		Some(())
	});

	view!(scope,
		<div class="frames" _ref=container>
			// Purposely iterates over indexes to re-use existing nodes
			<For
				each=move || 0..viewports()
				key=|&index| index
				view={move |index| view!(scope, <Frame index />)}
			/>
		</div>
	)
}
