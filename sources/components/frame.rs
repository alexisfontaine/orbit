use leptos::*;

use crate::model::Source;

use super::State;


/// Number of frames to be eagerly loaded, before and after the active one.
const EAGER_LOADED: usize = 16;

/// Number of frames to be fetched with a lower priority.
const LOW_PRIORITY_FETCHED: usize = EAGER_LOADED / 2;


#[component]
pub fn Frame (scope: Scope, index: usize) -> impl IntoView {
	let state = use_context::<State>(scope).unwrap();

	let camera = state.camera();
	let camera_index = state.camera;
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

	// FIXME: Split it into 2 derived signals
	let sources = create_memo(scope, move |_| camera.with(|camera| {
		camera.viewports[index]
			.source.as_ref()
			.map(|source| match source {
				Source::Dynamic(template) => (
					template.replace("{}", "3840"),
					vec![
						(500, template.replace("{}", "500")),
						(1_000, template.replace("{}", "1000")),
						(1_500, template.replace("{}", "1500")),
						(2_000, template.replace("{}", "2000")),
					]
				),

				Source::Static(values) => {
					let mut values = values.clone();

					(
						values.remove_entry(&0).or_else(|| values.pop_last()).unwrap().1,
						values.into_iter().collect()
					)
				}
			})
			.unwrap_or_default()
	}));

	view!(scope,
		<picture class="frame">
			<For
				each=move || sources().1
				key=move |source| (camera_index.get_untracked(), viewport_index.get_untracked(), source.0)
				view={move |source: (usize, String)| view!(scope,
					<source
						media=format!("(max-width:{}px)", &source.0)
						srcset=source.1
					/>
				)}
			/>

			<img
				class="frame_image"
				fetchpriority=priority
				loading=loading
				src=move || sources().0
			/>
		</picture>
	)
}
