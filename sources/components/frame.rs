use leptos::*;

use crate::state::use_state;


/// Number of frames to be eagerly loaded, before and after the active one.
const EAGER_LOADED: usize = 16;

/// Number of frames to be fetched with a lower priority.
const LOW_PRIORITY_FETCHED: usize = EAGER_LOADED / 2;


#[component]
pub fn Frame (scope: Scope, viewport: usize) -> impl IntoView {
	let state = use_state(scope);

	let loading = move || (EAGER_LOADED + 1..state.viewports() - EAGER_LOADED)
		.contains(&state.get_viewport().abs_diff(viewport))
		.then_some("lazy");

	let priority = move || {
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
	};

	view!(scope,
		<picture>
			// Purposely iterates over indexes to re-use existing nodes
			<For
				each=move || state.with_viewports(|viewports| 1..viewports[viewport].sources.len())
				key=|&index| index
				view=move |index| view!(scope,
					<source
						media=move || state.with_viewports(|viewports| {
							format!("(max-width:{}px)", viewports[viewport].sources[index].0)
						})
						srcset=move || state.with_viewports(|viewports| {
							viewports[viewport].sources[index].1.clone()
						})
					/>
				)
			/>

			<img
				class="frame"
				fetchpriority=priority
				loading=loading
				src=move || state.with_viewports(|viewports| {
					viewports[viewport].sources.get(0).cloned().unwrap_or_default().1
				})
			/>
		</picture>
	)
}
