use leptos::*;

use crate::state::use_state;


#[component]
fn Path (scope: Scope, width: f64, height: Signal<f64>, index: usize, class: String) -> impl IntoView {
	let state = use_state(scope);

	view!(scope,
		<path
			class=format!("shape {class}")
			d=move || {
				let shape = &state.get_scene().shapes[index];

				state.with_viewport(|viewport| shape.path(width, height(), &viewport.matrix))
			}
		/>
	)
}

#[component]
pub fn Overlay (scope: Scope) -> impl IntoView {
	const WIDTH: f64 = 1_000.;


	let state = use_state(scope);

	let height = Signal::derive(scope, move || state.with_camera(|camera| {
		let value = camera.aspect_ratio.split_once('/').unwrap();

		(WIDTH / value.0.parse::<f64>().unwrap() * value.1.parse::<f64>().unwrap()).ceil()
	}));

	view!(scope,
		<svg
			class="overlay"
			viewBox=move || format!("0 0 {WIDTH:.0} {:.0}", height())
			xmlns="http://www.w3.org/2000/svg"
		>
			// FIXME: Make use of the `For` component
			{move || state.with_camera(|camera| camera.styles
				.iter()
				.map(|style| view!(scope,
					<Path 
						class=style.name.clone()
						height
						index=style.index
						width=WIDTH
					/>
				))
				.collect::<Vec<_>>())}
		</svg>
	)
}
