use leptos::*;

use super::State;


/// Number of frames to be eagerly loaded, before and after the active one.
const EAGER_LOADED: usize = 16;

/// Number of frames to be fetched with a lower priority.
const LOW_PRIORITY_FETCHED: usize = EAGER_LOADED / 2;


#[component]
pub fn Frame (scope: Scope, index: usize) -> impl IntoView {
	let state = use_context::<State>(scope).unwrap();

	let camera = state.camera();
	let viewport_index = state.viewport;
	let viewports = state.viewports();

	let loading = move || (EAGER_LOADED + 1..viewports() - EAGER_LOADED)
		.contains(&viewport_index.get().abs_diff(index))
		.then_some("lazy");

	let priority = move || {
		let delta = viewport_index.get().abs_diff(index);

		if delta == 0 {
			Some("high")
		} else {
			let length = viewports();

			(
				(LOW_PRIORITY_FETCHED + 1..length - LOW_PRIORITY_FETCHED).contains(&delta) &&
				!(EAGER_LOADED + 1..length - EAGER_LOADED).contains(&delta)
			).then_some("low")
		}
	};

	let sources = Signal::derive(scope, move || camera.with(|camera| camera.viewports[index].sources.clone()));

	view!(scope,
		<picture>
			// Purposely iterates over indexes to re-use existing nodes
			<For
				each=move || 1..sources.with(|sources| sources.len())
				key=|&index| index
				view=move |index| view!(scope,
					<source
						media=move || format!("(max-width:{}px)", sources.with(|sources| sources[index].0))
						srcset=move || sources.with(|sources| sources[index].1.clone())
					/>
				)
			/>

			<img
				class="frame"
				fetchpriority=priority
				loading=loading
				src=move || sources.with(|sources| sources.get(0).cloned()).unwrap_or_default().1
			/>
		</picture>
	)
}
