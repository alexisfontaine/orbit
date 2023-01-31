use leptos::*;

use crate::model::Frame;
use crate::state::use_viewer_state;


/// Number of frames to be eagerly loaded, before and after the active one.
const EAGER_LOADED: usize = 16;

/// Number of frames to be fetched with a lower priority.
const LOW_PRIORITY_FETCHED: usize = EAGER_LOADED / 2;


#[component]
pub fn Frame (scope: Scope, viewport: usize) -> impl IntoView {
	let state = use_viewer_state(scope);
	let loading = create_rw_signal(scope, true);

	let fallback = move || state.with_viewports(|viewports| viewports[viewport].frames
		.iter()
		.position(Frame::is_fallback));

	let source = move || state.with_viewports(|viewports|
		Some(viewports[viewport].frames[fallback()?].source.clone()));

	create_effect(scope, move |previous| {
		let current = source();

		if !previous.contains(&current) && !loading.get() {
			loading.set(true);
		}

		current
	});

	view!(scope,
		<picture>
			// Purposely iterates over indexes to re-use existing nodes
			<For
				each=move || {
					let fallback = fallback();

					state
						.with_viewports(|viewports| 0..viewports[viewport].frames.len())
						.filter(move |index| !fallback.contains(index))
				}
				key=|&index| index
				view=move |index| view!(scope,
					<source
						media=move || state.with_viewports(|viewports|
							viewports[viewport].frames[index].size.map(|size| format!("(max-width:{size}px)")))
						srcset=move || state.with_viewports(|viewports|
 							viewports[viewport].frames[index].source.clone())
						type=move || state.with_viewports(|viewports|
 							viewports[viewport].frames[index].kind.clone())
					/>
				)
			/>

			<img
				class="frame"
				class:loading=loading
				fetchpriority=move || {
					let delta = state.get_viewport().abs_diff(viewport);

					if delta == 0 {
						Some("high")
					} else {
						let length = state.viewports();

						(
							(LOW_PRIORITY_FETCHED + 1..length - LOW_PRIORITY_FETCHED).contains(&delta) &&
							!(EAGER_LOADED + 1..length - EAGER_LOADED).contains(&delta)
						).then_some("low")
					}
				}
				loading=move || (EAGER_LOADED + 1..state.viewports() - EAGER_LOADED)
					.contains(&state.get_viewport().abs_diff(viewport))
					.then_some("lazy")
				on:load=move |_| if loading.get_untracked() {
					loading.set(false);
				}
				src=source
			/>
		</picture>
	)
}
