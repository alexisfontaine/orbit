use std::iter::once;

use leptos::*;

use crate::model::Source;

use super::State;


/// Number of frames to be eagerly loaded, before and after the active one.
const EAGER_LOADED: usize = 16;

/// Number of frames to be fetched with a lower priority.
const LOW_PRIORITY_FETCHED: usize = EAGER_LOADED / 2;


#[component]
pub fn Frame (scope: Scope, index: usize) -> Element {
	let state = use_context::<State>(scope).unwrap();

	let length = {
		let state = state.clone();

		create_memo(scope, move |_| state.viewports().len())
	};

	let loading = move || (EAGER_LOADED + 1..length() - EAGER_LOADED)
		.contains(&state.viewport.get().abs_diff(index))
		.then_some("lazy");

	let priority = move || {
		let delta = state.viewport.get().abs_diff(index);

		if delta == 0 {
			Some("high")
		} else {
			let length = length();

			(
				(LOW_PRIORITY_FETCHED + 1..length - LOW_PRIORITY_FETCHED).contains(&delta) &&
				!(EAGER_LOADED + 1..length - EAGER_LOADED).contains(&delta)
			).then_some("low")
		}
	};

	let sources = create_memo(scope, move |_| {
		let viewports = &state.viewports();

		viewports[index]
			.source.as_ref()
			.map(|source| match source {
				Source::Dynamic(template) => vec![
					(0, template.replace("{}", "3840")),
					(500, template.replace("{}", "500")),
					(1_000, template.replace("{}", "1000")),
					(1_500, template.replace("{}", "1500")),
					(2_000, template.replace("{}", "2000")),
				],

				Source::Static(values) => {
					let mut values = values.clone();

					once(values.remove_entry(&0).or_else(|| values.pop_last()).unwrap())
						.chain(values)
						.collect()
				}
			})
			.unwrap_or_default()
	});

	view!(scope,
		<picture class="frame">
			<For each=move || (0..sources().len()).skip(1).collect() key=|index| *index>
				{move |scope, &index: &usize| view!(scope, 
					<source 
						media=move || format!("(max-width:{}px)", &sources()[index].0)
						srcset=move || sources()[index].1.clone()
					/>
				)}
			</For>

			<img 
				class="frame_image" 
				fetchpriority=priority 
				loading=loading 
				src=move || sources()[0].1.clone()
			/>
		</picture>
	)
}
